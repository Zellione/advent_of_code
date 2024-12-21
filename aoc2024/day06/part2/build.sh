#!/bin/bash

cmake -GNinja -DCMAKE_BUILD_TYPE=Debug -Bbuild/default/

pushd ./build/default
ninja
popd
