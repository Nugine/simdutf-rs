#!/usr/bin/env python3
from pathlib import Path
import subprocess
import shutil

import requests
import typer
import re

cli = typer.Typer()

TEMP_DIR = Path("temp")
SIMDUTF_DIR = TEMP_DIR / "simdutf"


def shell(cmd: str, cwd: Path | None = None, check: bool = True):
    print(cmd, flush=True)
    subprocess.run(cmd, check=check, shell=True, cwd=cwd)


# https://github.com/simdutf/simdutf/releases
def get_latest_version() -> str:
    repo = "simdutf/simdutf"
    url = f"https://github.com/{repo}/releases/latest"
    r = requests.get(url)
    version = r.url.split("/")[-1]
    return version


@cli.command()
def check():
    vendor_version = None
    with open("cpp/simdutf.h", "r") as f:
        for line in f.readlines():
            line = line.strip()
            if line.startswith("#define SIMDUTF_VERSION"):
                vendor_version = "v" + line.split()[-1][1:-1]
    assert vendor_version is not None

    latest_version = get_latest_version()
    print(f"vendor version: {vendor_version}", flush=True)
    print(f"latest version: {latest_version}", flush=True)

    return vendor_version, latest_version


@cli.command()
def download():
    TEMP_DIR.mkdir(exist_ok=True)
    shutil.rmtree(SIMDUTF_DIR, ignore_errors=True)

    git_url = "https://github.com/simdutf/simdutf.git"
    depth = 50
    shell(f"git clone {git_url} -b master --depth={depth}", cwd=TEMP_DIR)


def postprocess(src: Path, dst: Path):
    with open(dst, "w") as dst_file:
        with open(src, "r") as src_file:
            for line in src_file.read().splitlines():
                if re.match(r"^/\* auto-generated on.+$", line):
                    continue

                if re.match(r"^// /.+simdutf-rs.+simdutf/include.+\.h:.+$", line):
                    continue

                if re.match(r"^// /.+simdutf-rs.+simdutf/src.+\.cpp:.+$", line):
                    continue

                # Make SIMDUTF_FEATURE_* defines conditional so that
                # they can be overridden from compiler flags (build.rs).
                m = re.match(r"^#define (SIMDUTF_FEATURE_\w+) 1$", line)
                if m:
                    name = m.group(1)
                    dst_file.write(f"#ifndef {name}\n")
                    dst_file.write(line + "\n")
                    dst_file.write("#endif\n")
                    continue

                dst_file.write(line + "\n")


@cli.command()
def vendor():
    shell("python3 ./singleheader/amalgamate.py", cwd=SIMDUTF_DIR)
    postprocess(SIMDUTF_DIR / "singleheader/simdutf.h", Path("cpp/simdutf.h"))
    postprocess(SIMDUTF_DIR / "singleheader/simdutf.cpp", Path("cpp/simdutf.cpp"))


@cli.command()
def upgrade(force: bool = False, pr: bool = False):
    vendor_version, latest_version = check()

    if vendor_version == latest_version and not force:
        print("already up to date", flush=True)
        return

    download()
    shell(f"git checkout {latest_version}", cwd=SIMDUTF_DIR)

    if pr:
        branch = f"auto/upgrade/{latest_version}"

        try:
            shell(f"git fetch origin {branch}")
            has_remote = True
        except Exception:
            has_remote = False

        if has_remote:
            shell(f"git checkout {branch}")
            shell("git reset --hard origin/main")
        else:
            shell(f"git checkout -b {branch}")

        vendor()

        shell("git add -A")
        shell(f"git commit -m 'upstream {latest_version}'")
        shell(f"git push origin -f --set-upstream {branch}")

        shell(
            f"gh pr create -B main -H {branch}"
            f" --title 'Upgrade simdutf to {latest_version}'"
            f" --body 'https://github.com/simdutf/simdutf/releases/tag/{latest_version}'",
            check=False,
        )
    else:
        vendor()

    print("Done", flush=True)


if __name__ == "__main__":
    cli()
