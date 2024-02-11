pub mod parser;
use parser::*;

extern crate clang;
use clang::*;


fn main() {
    let file_path = "test/main.c";

    let clng = Clang::new().unwrap();
    let index = Index::new(&clng, false, false);

    let tu = index.parser(file_path).parse().unwrap();

    let mut parser = CodeParser::new();
    parser.parse_code(tu.get_entity());

    println!("Variables: {:?}", parser);
}
