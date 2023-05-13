#!/usr/bin/env python3
import requests

repo = "simdutf/simdutf"

url = f"https://github.com/{repo}/releases/latest"
r = requests.get(url)
version = r.url.split("/")[-1]

current = version
last = "v3.2.9"
print(f"current: {current}")
print(f"last:    {last}")
assert current == last
