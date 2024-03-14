use std::collections::HashMap;
use clang::{Entity, EntityKind};

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub size: usize,
    pub var_type: String,
}

#[derive(Debug)]
pub struct CodeParser {
    variables: HashMap<String, Variable>,
}

impl Variable {
    pub fn new(var_name: String, var_size: usize, var_type: String) -> Self {
        Variable {
            name: var_name,
            size: var_size,
            var_type: var_type,
        }
    }
}

impl CodeParser {
    pub fn new() -> Self {
        CodeParser {
            variables: HashMap::new(),
        }
    }

    pub fn add_new_variable(&mut self, var_name: String, size: usize, var_type: String) {
        self.variables.insert(var_name.clone(), Variable::new(var_name, size, var_type));
    }

    fn parse_strcpy(&self, args: &Vec<Entity<'_>>) -> bool {
        let dest = args[0];
        let srce = args[1];

        let var_dest = self.variables.get(&dest.get_display_name().unwrap()).expect("Variable not declared");
        let var_srce = self.variables.get(&srce.get_display_name().unwrap()).expect("Variable not declared");

        var_srce.size >= var_dest.size
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
                        println!("name: {:?}, size: {:?}, type: {:?}", type_name, size, type_name);
                        self.add_new_variable(entity.get_name()
                            .unwrap_or("Variable".to_string()), size, type_name.to_string());
                    },
                    clang::TypeKind::Pointer => {
                        // Only for string literal initialisation or empty declaration
                        let type_name = tpe.get_display_name();
                        if let Some(child) = entity.get_child(0) {
                            let value = child.get_child(0).unwrap().get_display_name().unwrap_or_default();
                            let size = value.len() - 1; // Remove "" and add \0 in count
                            println!("Found value: {}, size: {}, displ name: {}", value, size, type_name);
                            self.add_new_variable(entity.get_name()
                                .unwrap_or("Variable".to_string()), size, type_name)
                        } else {
                            self.add_new_variable(entity.get_name()
                                .unwrap_or("Variable".to_string()), 0, type_name)
                        }

                        // TODO: Multiply size by sizeof(type_name)
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
                if display_name == "strcpy" {
                    if let Some(args) = entity.get_arguments() {
                        let found_buff_overflow = self.parse_strcpy(&args);
                        if found_buff_overflow {
                            println!("Detected buffer overflow at line {:?}", entity.get_location().unwrap().get_file_location());
                        }
                    }
                }
                // println!("Function call: {}", display_name);

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
