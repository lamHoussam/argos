# A Rust Guard for Overflow Security (ARGOS)

## Setup

### Install Rust

You need to have Rust installed on your machine and setup. Here is a link to the official Rust installation guide: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Install Clang and LLVM

Run the following command to install clang and llvm

```shell
sudo apt-get install llvm clang
```

## Run

To run the program, you need to build the rust dynamic library and the shared library, using the following command:

```shell
./build.sh
```

and then you can run the program using the following command. Here is the help message for the program.

```shell
Usage: rust_overflow_sentinel [OPTIONS] --mode <MODE> --file-path <FILE_PATH>

Options:
  -m, --mode <MODE>            Mode: static | dynamic
  -f, --file-path <FILE_PATH>  Path to the file to check
  -h, --help                   Print help
```

To run the program in static mode, use the following command:

```shell
cargo run --release -- -m static -f <path_to_c_source>
```

To run the program in dynamic mode, use the following command:

```shell
cargo run --release -- -m dynamic -f <path_to_binary>
```
