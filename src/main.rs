pub mod parser;
use parser::*;

extern crate clang;
use clang::*;
use std::os::unix::thread;
use std::{env, process::Command};

// use std::sync::{Arc, Mutex};
// 
// use lazy_static::lazy_static;
// 
// lazy_static! {
//     #[no_mangle]
//     static ref MYVAR: Arc<Mutex<i32>> = Arc::new(Mutex::new(0)); 
// }
//
//

use std::thread as other_thread;
use std::time::Duration;

fn main() {
    // let test = true;
    // if test {

    //     {
    //         let mut num = MYVAR.lock().unwrap();
    //         *num += 1;
    //     }

    //     println!("Freed with value: {}", MYVAR.lock().unwrap());

    //     return;
    // }



    let dynamic: bool = true;
    other_thread::sleep(Duration::from_secs(2));

    if dynamic {
        let target_binary = "test/main";
        let library_path = env::current_dir().unwrap().join("src/libintercept.so");

        {
            let output = Command::new(target_binary)
//                 .env("LD_PRELOAD", library_path)
                .output();
            println!("Output: {:?}", output);

            // other_thread::sleep(Duration::from_secs(2));
        }

        parser::print_myvar();

        return;
    }

    let file_path = "test/main.c";

    let clng = Clang::new().unwrap();
    let index = Index::new(&clng, false, false);
    let tu = index.parser(file_path).parse().unwrap();

    let mut parser = CodeParser::new();
    for entity in tu.get_entity().get_children() {
        if let Some(location) = entity.get_location() {
            if location.is_in_main_file() { parser.parse_code(&entity); }
        }
    }

    println!("Variables: {:?}", parser);
}
