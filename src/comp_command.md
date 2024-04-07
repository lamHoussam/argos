# Commands

## First command
cargo build --release
clang -shared -fPIC src/intercept.c -o src/libintercept.so -L./target/release -lintercept_test
export LD_LIBRARY_PATH=./target/release/
export LD_PRELOAD=./src/libintercept.so 
clang -o src/main src/main.c


## Second Command

cargo build --release
clang test/main.c -o test/main.o -L./target/release -lrust_overflow_sentinel
./test/main || cargo run --release
