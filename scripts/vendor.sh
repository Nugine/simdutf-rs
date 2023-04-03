#!/bin/bash -ex
pushd ./simdutf
    python3 ./singleheader/amalgamate.py
popd

cp ./simdutf/singleheader/simdutf.h   ./cpp
cp ./simdutf/singleheader/simdutf.cpp ./cpp

pushd ./cpp
    sed -i 's/^\/\/ dofile.*//g' simdutf.h
    sed -i 's/^\/\/ dofile.*//g' simdutf.cpp
popd
