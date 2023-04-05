#!/bin/bash -ex
pushd ./simdutf
    python3 ./singleheader/amalgamate.py
popd

cp ./simdutf/singleheader/simdutf.h   ./crates/simdutf/cpp
cp ./simdutf/singleheader/simdutf.cpp ./crates/simdutf/cpp

pushd ./crates/simdutf/cpp
    sed -i 's/^\/\/ dofile.*//g' simdutf.h
    sed -i 's/^\/\/ dofile.*//g' simdutf.cpp
popd
