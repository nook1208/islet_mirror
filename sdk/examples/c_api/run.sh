#!/bin/bash

set -e

cd ../../
cargo build
cd -
g++ attestation.cc \
	-lislet_sdk \
	-L ../../../out/x86_64-unknown-linux-gnu/debug/
LD_LIBRARY_PATH=../../../out/x86_64-unknown-linux-gnu/debug/ ./a.out

g++ sealing.cc \
	-lislet_sdk \
	-L ../../../out/x86_64-unknown-linux-gnu/debug/
LD_LIBRARY_PATH=../../../out/x86_64-unknown-linux-gnu/debug/ ./a.out