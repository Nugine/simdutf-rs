#!/bin/bash -ex
pushd simdutf
    python3 ./singleheader/amalgamate.py
popd

cp simdutf/singleheader/simdutf.h   ./src/cpp
cp simdutf/singleheader/simdutf.cpp ./src/cpp

pushd src/cpp
    sed -i 's/^\/\/ dofile.*//g' simdutf.h
    sed -i 's/^\/\/ dofile.*//g' simdutf.cpp
popd
