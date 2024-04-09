use core::panic;
use std::{env, isize};
use std::collections::HashMap;
use std::error::Error;
use libc::c_void;
use regex::Regex;
use clang::{Entity, EntityKind};
// use std::process;
use std::sync::{Mutex, Arc};
use lazy_static::lazy_static;

extern crate libc;

use std::ptr;

lazy_static! {
    #[no_mangle]
    static ref CODEPARSER: Mutex<CodeParser> = Mutex::new(CodeParser::new()); 
}

lazy_static! {
    #[no_mangle]
    pub static ref MYVAR: Arc<Mutex<i32>> = Arc::new(Mutex::new(0)); 
}

pub struct SharedState {
    pub value: Mutex<i32>
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct TestStruct {
    pub value: i32,
}

pub fn write_to_shared_memory(data: TestStruct) -> i32 {
    let key = 69420;
    let mem_size = std::mem::size_of::<TestStruct>() as libc::size_t;
    let shm_id = unsafe { libc::shmget(key, mem_size, libc::IPC_CREAT | 0o666) };
    println!("get shmem id {}", shm_id);
    if shm_id < 0 {
        panic!("Failed to write to shmem");
    }
    let ptr = unsafe { libc::shmat(shm_id, ptr::null() as *const libc::c_void, 0) as *mut TestStruct};
    println!("attach shmem {:?}", ptr);
    if (ptr as isize) == -1{
        panic!("Failed to attach to shmem {}", std::io::Error::last_os_error());
    }

    unsafe {
        ptr::write(ptr, data);
        // ptr.copy_from_nonoverlapping(data, 1);
        // *ptr = data;
        println!("Write to shmem");
        libc::shmdt(ptr as *const c_void);
    }
    shm_id
}

pub fn read_from_shmem(shm_id: i32) -> TestStruct {
    let ptr = unsafe { libc::shmat(shm_id, ptr::null(), 0) } as *mut TestStruct;
    if ptr.is_null() {
        panic!("Failed to attach to shmem");
    }
    let data = unsafe { *ptr };
    unsafe {
        // shmat(shm_id, ptr as *mut libc::c_void, 0);
        libc::shmdt(ptr as *const _ as *mut libc::c_void);
        libc::shmctl(shm_id, libc::IPC_RMID, ptr::null_mut());
    }
    data
}


// lazy_static! {
//     pub static ref SHARED_MEMORY: SharedState 
// }

// TODO: Replace var_type with sizeof_type
#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub size: usize,
    pub var_type: String,
    pub max_bounds_checked: usize,
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
            max_bounds_checked: 100,
        }
    }
}

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

pub fn get_static_code_parser() -> &'static Mutex<CodeParser> {
    &CODEPARSER
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

        match srce.get_display_name() {
            Some(var_name) => {
                let var_srce = self.variables.get(&var_name).expect("Variable not declared");
                var_srce.size >= var_dest.size
            },
            None => {
                let value = get_litteral(srce);
                println!("Litteral: {}", value);
                let size = value.len() - 2;
                size >= var_dest.size
            },
        }
    }

    fn parse_scanf(&self, args: &Vec<Entity<'_>>) -> bool {
        let it = args[0];
        let format = get_litteral(it);
        println!("Format: {}", format);

        let re = Regex::new(r"%(\d+)?s").unwrap();
        let mut arg_iter = args.iter();
        arg_iter.next();

        for cap in re.captures_iter(&format) {
            let buffer_size = match cap.get(1) {
                Some(matched) => matched.as_str().parse::<usize>().unwrap_or(0),
                None => 0,
            };

            println!("Found: {}, size: {}", &cap[0], buffer_size);
            match arg_iter.next() {
                Some(value) => {
                    let var_name = value.get_display_name().unwrap();
                    let var = self.variables.get(&var_name).expect("Variable not declared!");
                    if var.size >= buffer_size { return true; }
                    // println!("Goes with {}, size: {}", var_name, );
                },
                None => break,
            }
        }

        false
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
                            println!("Detected buffer overflow pattern at line {:?}", entity.get_location().unwrap().get_file_location());
                        } else {
                            println!("WARNING: Use of unsafe function strcpy");
                        }
                    }
                }
                else if display_name == "strcat" {
                    if let Some(args) = entity.get_arguments() {
                        let found_buff_overflow = self.parse_strcpy(&args);
                        if found_buff_overflow {
                            println!("Detected buffer overflow at line {:?}", entity.get_location().unwrap().get_file_location());
                        } else {
                            println!("WARNING: Use of unsafe function strcat");
                        }
                    }
                } else if display_name == "scanf" {
                    if let Some(args) = entity.get_arguments() {
                        let found_buff_overflow = self.parse_scanf(&args);
                        if found_buff_overflow {
                            println!("Detected buffer overflow at line {:?}", entity.get_location().unwrap().get_file_location());
                        } else {
                            println!("WARNING: Use of unsafe function strcat");
                        }
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
