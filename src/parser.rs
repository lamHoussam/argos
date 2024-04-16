use clang::{source::SourceRange, Entity, EntityKind};
use std::collections::HashMap;
use crate::lib::Variable;

#[derive(Debug)]
/// Struct to parse the C code, holds variables and their values
pub struct CodeParser {
    variables: HashMap<String, Variable>,
}

/// Get the litteral String value of an entity
fn get_litteral(entity: Entity<'_>) -> String {
    let mut it = entity;

    loop {
        if let Some(litt) = it.get_display_name() { return litt; } 
        else { 
            match it.get_child(0) {
                Some(iter) => it = iter,
                None => (),
            }
        }
    }
}

impl CodeParser {
    pub fn new() -> Self {
        CodeParser {
            variables: HashMap::new(),
        }
    }

    pub fn add_new_variable(&mut self, var_name: String, size: usize) {
        self.variables.insert(var_name.clone(), Variable::new(var_name, size));
    }

    /// Parse the strcpy function
    fn parse_strcpy(&self, args: &Vec<Entity<'_>>, function_range: SourceRange) {
        let dest = args[0];
        let srce = args[1];

        let var_dest = self.variables.get(&dest.get_display_name().unwrap()).expect("Variable not declared");

        let bytes_offset_start = function_range.get_start();
        let bytes_offset_end = function_range.get_end();

        let mut found_buff_overflow = false;
        let mut replacement = String::new();

        match srce.get_display_name() {
            Some(var_name) => {
                let var_srce = self.variables.get(&var_name).expect("Variable not declared");
                replacement = format!("strncpy({}, {}, {})", args[0].get_display_name().unwrap(), args[1].get_display_name().unwrap(), var_dest.size - 1);
                found_buff_overflow = var_srce.size >= var_dest.size;
            },
            None => {
                let value = get_litteral(srce);
                // println!("Litteral: {}", value);
                replacement = format!("strncpy({}, {}, {})", args[0].get_display_name().unwrap(), value, var_dest.size);
                let size = value.len() - 2;
                found_buff_overflow = size >= var_dest.size;
            },
        }

        if found_buff_overflow {
            println!("Detected buffer overflow pattern at line {:?}", bytes_offset_start.get_file_location().line);
            println!("\tReplace with: {}", replacement);
            println!("\tReplace this range of bytes: {:?} -> {:?}", bytes_offset_start.get_file_location().offset, bytes_offset_end.get_file_location().offset);
        } else {
            println!("WARNING: Use of unsafe function strcpy");
        }
    }

    fn parse_scanf(&self, args: &Vec<Entity<'_>>, range: SourceRange) -> bool {
        let it = args[0];
        let format = get_litteral(it);
        println!("Format: {}", format);

        let mut found_vuln = false;
        let mut replacement = String::new();

        let re = regex::Regex::new(r"%(\d+)?s").unwrap();
        let mut arg_iter = args.iter();
        arg_iter.next();

        for cap in re.captures_iter(&format) {
            let buffer_size = match cap.get(1) {
                Some(matched) => matched.as_str().parse::<usize>().unwrap_or(0),
                // TODO: Manage this case
                None => usize::MAX,
            };

            println!("Found: {}, size: {}", &cap[0], buffer_size);
            match arg_iter.next() {
                Some(value) => {
                    let var_name = value.get_display_name().unwrap();
                    let var = self.variables.get(&var_name).expect("Variable not declared!");
                    if var.size >= buffer_size { 
                        replacement = format!("scanf(%{}s, {})", buffer_size - 1, var_name);
                        found_vuln = true; 
                    }
                    // println!("Goes with {}, size: {}", var_name, );
                },
                None => break,
            }
        }

        if found_vuln {
            let bytes_offset_start = range.get_start();
            let bytes_offset_end = range.get_end();

            println!("Detected buffer overflow at line {:?}", bytes_offset_start.get_file_location().line);
            println!("\tReplace with: {}", replacement);
            println!("\tReplace this range of bytes: {:?} -> {:?}", bytes_offset_start.get_file_location().offset, bytes_offset_end.get_file_location().offset);
        } else {
            println!("WARNING: Use of unsafe function strcat");
        }
        found_vuln
    }

    pub fn parse_code(&mut self, entity: &Entity<'_>) {

        // println!("Name: {:?}; Kind: {:?}", entity.get_name(), entity.get_kind());  
        // println!("Ent: {:?}", entity);

        match entity.get_kind() {
            EntityKind::VarDecl => {
                let tpe = entity.get_type().unwrap();
                match tpe.get_kind() {
                    clang::TypeKind::ConstantArray => {
                        // TODO: Cleaup
                        let display_name = tpe.get_display_name();
                        let display_name_split: Vec<&str> = display_name.split('[').collect();
                        let type_name = display_name_split.first().unwrap();
                        let size_str = display_name_split.last().unwrap().trim_end_matches(']');
                        let size = size_str.parse::<usize>().unwrap_or_default();

                        // TODO: Multiply size by sizeof(type_name)
                        println!("name: {:?}, size: {:?}, type: {:?}", entity.get_name(), size, type_name);
                        self.add_new_variable(entity.get_name()
                            .unwrap_or("Variable".to_string()), size);
                    },
                    clang::TypeKind::Pointer => {
                        // Only for string literal initialisation or empty declaration
                        let type_name = tpe.get_display_name();
                        if let Some(child) = entity.get_child(0) {
                            let value = child.get_child(0).unwrap().get_display_name().unwrap_or_default();
                            let size = value.len() - 1; // Remove "" and add \0 in count
                            println!("Found value: {}, size: {}, displ name: {}", value, size, type_name);
                            self.add_new_variable(entity.get_name()
                                .unwrap_or("Variable".to_string()), size)
                        } else {
                            self.add_new_variable(entity.get_name()
                                .unwrap_or("Variable".to_string()), 0)
                        }

                        // println!("name: {:?}", display_name);
                    }, 
                    _ => {
                        
                    }
                }

                // println!("Entity: {:?}", tpe);
                // println!("Type: {:?}, other: {:?}", tpe, tpe.get_kind());
                // println!("=> Var Entity: {:?}, Type: {:?}", entity, entity.get_type());

                // println!("-> Children: {:?}", entity.get_children());

            }, 
            EntityKind::CallExpr => {
                // let tpe = entity.get_type().unwrap();
                let display_name = entity.get_display_name().unwrap();
                let range = entity.get_range().unwrap();
                // println!("Function call: {}, range: {:?}", display_name, range);
                
                // TODO: Use pattern matching
                if let Some(args) = entity.get_arguments() {
                    if display_name == "strcpy" {
                        self.parse_strcpy(&args, range);
                    } else if display_name == "strcat" {
                        self.parse_strcpy(&args, range);
                    } else if display_name == "scanf" {
                        self.parse_scanf(&args, range);
                    }
                }
                // println!("Function call: {}", display_name);

            }, 

            // TODO: Check in if statements if variables bounds have been checked
            EntityKind::IfStmt => {
                let _display_name = entity.get_display_name().unwrap();

            },
            _ => {
                
            }
        }
        let children = &entity.get_children();
        if children.len() == 0 { return; }

        for e in children {
            self.parse_code(e);
        }
    }
}
