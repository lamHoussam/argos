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
    for entity in tu.get_entity().get_children() {
        if let Some(location) = entity.get_location() {
            if location.is_in_main_file() { parser.parse_code(&entity); }
        }
    }

    println!("Variables: {:?}", parser);
}
