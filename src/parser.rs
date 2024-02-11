use std::collections::HashMap;
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

    pub fn add_new_variable(&mut self, var_name: String) {
        self.variables.insert(var_name.clone(), Variable::new(var_name, 16));        
    }

    pub fn parse_code(&mut self, entity: &Entity<'_>) {
        if entity.get_kind() == EntityKind::VarDecl {
            let tpe = entity.get_type().unwrap();
            match tpe.get_kind() {
                clang::TypeKind::ConstantArray => {
                    
                },
                _ => {

                }
            }

            println!("Entity: {:?}", tpe);
            // println!("Type: {:?}, other: {:?}", tpe, tpe.get_kind());
            // println!("=> Var Entity: {:?}, Type: {:?}", entity, entity.get_type());
            self.add_new_variable(entity.get_name().unwrap_or("Houssam".to_string()));

            // println!("-> Children: {:?}", entity.get_children());
        }
        
        let children = &entity.get_children();
        if children.len() == 0 { return; }

        for e in children {
            self.parse_code(e);
        }
    }
}
