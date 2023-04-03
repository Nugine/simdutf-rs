#!/bin/bash -ex
if [ ! -d "simdutf" ]; then
    git clone https://github.com/simdutf/simdutf.git -b master --depth=5
else
    pushd simdutf
    git pull
    popd
fi
