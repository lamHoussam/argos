pub mod parser;
use parser::CodeParser;

extern crate clang;
use clang::{Clang, Index};

use clap::Parser as ClapParser;

pub mod lib;
use lib::{read_from_shmem, write_to_new_shmem, detach_shmem, DynamicPtrTracker};

 

#[derive(ClapParser, Debug)]
#[command()]
struct Args {
    /// Mode: static | dynamic
    # [arg(short, long)]
    mode: String,

    /// path to the file to check
    # [arg(short, long)]
    file_path: String,
}


fn main() {
    let args = Args::parse();

    if args.mode == "static" {
        let clng = Clang::new().unwrap();
        let index = Index::new(&clng, false, false);
        let tu = index.parser(args.file_path).parse().expect("File not found");

        let mut parser = CodeParser::new();
        for entity in tu.get_entity().get_children() {
            if let Some(location) = entity.get_location() {
                if location.is_in_main_file() { parser.parse_code(&entity); }
            }
        }
    }
    else if args.mode == "dynamic" {
        let shm_key = 43;
        let shmem_id = write_to_new_shmem(DynamicPtrTracker::new(), shm_key);
        println!("Shmem ID: {:?}", shmem_id);

        let target_binary = args.file_path;
        let library_path = std::env::current_dir().unwrap().join("src/libintercept.so");
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

        println!("Output: {:?}", output);

        let mut test_struct = read_from_shmem::<DynamicPtrTracker>(shm_key);
        test_struct.check();
        println!("TestStruct: {:#?}", test_struct);
    }
    else {
        panic!("Mode should either be static or dynamic");
    }

}

/*
fn main() {

    let dynamic: bool = true;
    other_thread::sleep(Duration::from_secs(2));

    if dynamic {
        let target_binary = "test/main";
        // let library_path = env::current_dir().unwrap().join("src/libintercept.so");

        {
            let output = Command::new(target_binary)
//                 .env("LD_PRELOAD", library_path)
                .output();
            println!("Output: {:?}", output);

            // other_thread::sleep(Duration::from_secs(2));
        }

        // parser::print_myvar();

        return;
    }

    let file_path = "test/main.c";

    let clng = Clang::new().unwrap();
    let index = Index::new(&clng, false, false);
    let tu = index.parser(file_path).parse().unwrap();

    // let mut parser = CodeParser::new();
    // for entity in tu.get_entity().get_children() {
    //     if let Some(location) = entity.get_location() {
    //         if location.is_in_main_file() { parser.parse_code(&entity); }
    //     }
    // }

    // println!("Variables: {:?}", parser);
}
*/
