#!/usr/bin/env bash

cargo build --release
clang -shared -fPIC src/intercept.c -o src/libintercept.so -L./target/release -lrust_overflow_sentinel
export LD_PRELOAD=./src/libintercept.so
clang -o test/main test/main.c
