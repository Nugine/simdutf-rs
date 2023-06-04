#!/bin/bash -ex
pushd ./temp/simdutf
    python3 ./singleheader/amalgamate.py
popd

cp ./temp/simdutf/singleheader/simdutf.h   ./crates/simdutf/cpp
cp ./temp/simdutf/singleheader/simdutf.cpp ./crates/simdutf/cpp

pushd ./crates/simdutf/cpp
    sed -i 's/^\/\/ dofile.*//g' simdutf.h
    sed -i 's/^\/\/ dofile.*//g' simdutf.cpp
popd
