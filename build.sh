#!/usr/bin/env bash

clang -o test/static_demo test/static_demo.c
clang -o test/dynamic_demo test/dynamic_demo.c

cargo build --release
clang -shared -fPIC src/intercept.c -o src/libintercept.so -L./target/release -lrust_overflow_sentinel
export LD_PRELOAD=./src/libintercept.so
