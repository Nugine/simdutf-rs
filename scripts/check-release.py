#!/usr/bin/env python3
import requests

# https://github.com/simdutf/simdutf/releases
repo = "simdutf/simdutf"

url = f"https://github.com/{repo}/releases/latest"
r = requests.get(url)
version = r.url.split("/")[-1]

current = version

last = None
with open("crates/simdutf/cpp/simdutf.h", "r") as f:
    for line in f.readlines():
        line = line.strip()
        if line.startswith("#define SIMDUTF_VERSION"):
            last = "v" + line.split()[-1][1:-1]

print(f"current: {current}")
print(f"last:    {last}")
assert current == last
