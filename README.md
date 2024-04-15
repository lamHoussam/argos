# Buffer Overflow Analyzer

## Setup
Run the following command to install clang and llvm 
```shell
sudo apt-get install llvm clang
```

## Run
To run the project use the command at the root folder project
```shell
Usage: rust_overflow_sentinel [OPTIONS] --mode <MODE> --file-path <FILE_PATH>

Options:
  -m, --mode <MODE>            Mode: static | dynamic
  -f, --file-path <FILE_PATH>  Path to the file to check
  -h, --help                   Print help
```
