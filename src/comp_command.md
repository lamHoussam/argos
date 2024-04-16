# Commands

## First command
cargo build --release
clang -shared -fPIC src/intercept.c -o src/libintercept.so -L./target/release -lrust_overflow_sentinel
export LD_LIBRARY_PATH=./target/release/
export LD_PRELOAD=./src/libintercept.so 
clang -o src/main src/main.c


## Second Command

cargo build --release
clang -shared -fPIC src/intercept.c -o src/libintercept.so -L./target/release -lrust_overflow_sentinel
export LD_LIBRARY_PATH=./target/release/
clang test/main.c -o test/main.o -L./target/release -lrust_overflow_sentinel
./test/main || cargo run --release

## AST generation 
```shell
clang-check -ast-dump-filter=main test/static_demo.c
```