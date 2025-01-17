#!/bin/bash -ex
pushd ./temp/simdutf
    python3 ./singleheader/amalgamate.py
popd

cp ./temp/simdutf/singleheader/simdutf.h   ./cpp
cp ./temp/simdutf/singleheader/simdutf.cpp ./cpp

pushd ./cpp
    sed -i 's/^\/\/ dofile.*//g' simdutf.h
    sed -i 's/^\/\/ dofile.*//g' simdutf.cpp
popd
