pub mod parser;
use parser::CodeParser;

extern crate clang;
use clang::{Clang, Index};

use clap::Parser as ClapParser;

mod lib;
use crate::lib::{read_from_shmem, write_to_new_shmem, detach_shmem, DynamicPtrTracker};

#[derive(ClapParser, Debug)]
#[command()]
struct Args {
    /// Mode: static | dynamic
    # [arg(short, long)]
    mode: String,

    /// Path to the file to check
    # [arg(short, long)]
    file_path: String,
}

// TODO: Performance test

fn main() {
    let args = Args::parse();

    if args.mode == "static" {
        let clng = Clang::new().unwrap();
        let index = Index::new(&clng, false, false);
        let tu = index.parser(args.file_path.clone()).parse().expect("File not found");
        let file_content = std::fs::read_to_string(&args.file_path).expect("File not found");

        let c: Vec<u8> = file_content.bytes().collect();
        println!("File content: {:?}", c.get(148));

        let mut parser = CodeParser::new();
        for entity in tu.get_entity().get_children() {
            if let Some(location) = entity.get_location() {
                if location.is_in_main_file() { parser.parse_code(&entity); }
            }
        }
    }
    else if args.mode == "dynamic" {
        let shm_key = 42;
        let shmem_id = write_to_new_shmem(DynamicPtrTracker::new(), shm_key);
        println!("Shmem ID: {:?}", shmem_id);

        println!("-------------------STARTING DYNAMIC MODE-------------------");
        let target_binary = args.file_path;
        println!("Target binary: {:?}", target_binary);
        let library_path = std::env::current_dir().unwrap().join("src/libintercept.so");
        println!(">>> Loading library: {:?}", library_path);
        println!(">>> Starting binary: {:?}", library_path);

        let result_output = std::process::Command::new(target_binary)
            .env("LD_PRELOAD", library_path)
            .output();

        let output = match result_output {
            Ok(output) => output,
            Err(e) => {
                detach_shmem(shm_key);
                panic!("Failed to execute process: {}", e)
            },
        };

        println!("Output: {}", String::from_utf8(output.stdout).unwrap());

        let mut test_struct = read_from_shmem::<DynamicPtrTracker>(shm_key);
        detach_shmem(shm_key);
        test_struct.print_report();
    }
    else {
        panic!("Mode should either be static or dynamic");
    }

}
