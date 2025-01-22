#!/usr/bin/env python3
from pathlib import Path
import subprocess
import shutil

import requests
import typer

cli = typer.Typer()


def shell(cmd, cwd=None):
    print(cmd, flush=True)
    subprocess.run(cmd, check=True, shell=True, cwd=cwd)


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
    temp_dir = Path("temp")
    temp_dir.mkdir(exist_ok=True)

    simdutf_dir = temp_dir / "simdutf"
    shutil.rmtree(simdutf_dir, ignore_errors=True)

    git_url = "https://github.com/simdutf/simdutf.git"
    depth = 50
    shell(f"git clone {git_url} -b master --depth={depth}", cwd=temp_dir)

    return simdutf_dir


@cli.command()
def upgrade(pr: bool = False):
    vendor_version, latest_version = check()

    if vendor_version == latest_version:
        print("already up to date", flush=True)
        return

    simdutf_dir = download()
    shell(f"git checkout {latest_version}", cwd=simdutf_dir)

    shell("python3 ./singleheader/amalgamate.py", cwd=simdutf_dir)
    shutil.copy(simdutf_dir / "singleheader/simdutf.h", "cpp/simdutf.h")
    shutil.copy(simdutf_dir / "singleheader/simdutf.cpp", "cpp/simdutf.cpp")

    if pr:
        branch = f"auto/upgrade/{latest_version}"
        shell(f"git checkout -b {branch}")
        shell("git add -A")
        shell(f"git commit -m 'upstream {latest_version}'")
        shell(f"git push origin -f --set-upstream {branch}")
        shell(
            f"gh pr create -B main -H {branch}"
            f" --title 'Upgrade simdutf to {latest_version}'"
            f" --body 'https://github.com/simdutf/simdutf/releases/tag/{latest_version}'"
        )

    print("Done", flush=True)


if __name__ == "__main__":
    cli()
