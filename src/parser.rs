use std::{collections::HashMap, f32::consts::E};
use clang::{Entity, EntityKind};

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub size: usize,
}

#[derive(Debug)]
pub struct CodeParser {
    variables: HashMap<String, Variable>,
}

impl Variable {
    pub fn new(var_name: String, var_size: usize) -> Self {
        Variable {
            name: var_name,
            size: var_size,
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

    pub fn parse_code(&mut self, entity: &Entity<'_>) {

        // println!("Name: {:?}; Kind: {:?}", entity.get_name(), entity.get_kind());  
        // println!("Ent: {:?}", entity);

        if entity.get_kind() == EntityKind::VarDecl {
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
                    println!("name: {:?}, size: {:?}", type_name, size);
                    self.add_new_variable(entity.get_name()
                        .unwrap_or("Variable".to_string()), size);
                },
                clang::TypeKind::Pointer => {
                    // Only for string literal initialisation or empty declaration
                    let display_name = tpe.get_display_name();
                    if let Some(child) = entity.get_child(0) {
                        let value = child.get_child(0).unwrap().get_display_name().unwrap_or_default();
                        let size = value.len() - 1; // Remove "" and add \0 in count
                        println!("Found value: {}, size: {}", value, size);
                        self.add_new_variable(entity.get_name()
                            .unwrap_or("Variable".to_string()), size)
                    } else {
                        self.add_new_variable(entity.get_name()
                            .unwrap_or("Variable".to_string()), 0)
                    }

                    // TODO: Multiply size by sizeof(type_name)
                    println!("name: {:?}", display_name);
                }
                _ => {

                }
            }

            println!("Entity: {:?}", tpe);
            // println!("Type: {:?}, other: {:?}", tpe, tpe.get_kind());
            // println!("=> Var Entity: {:?}, Type: {:?}", entity, entity.get_type());

            // println!("-> Children: {:?}", entity.get_children());
        }
        
        let children = &entity.get_children();
        if children.len() == 0 { return; }

        for e in children {
            self.parse_code(e);
        }
    }
}
