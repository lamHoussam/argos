# Command
cargo build --release
clang -shared -fPIC src/intercept.c -o src/libintercept.so -L./target/releaes -lintercept_test
export LD_LIBRARY_PATH=./target/release/
export LD_PRELOAD=./src/libintercept.so 
clang -o src/main src/main.c
