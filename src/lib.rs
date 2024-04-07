use std::ffi::{c_char, CStr};

mod parser;
use parser::SHARED_MEMORY;
use std::sync::{Mutex, Arc};
use lazy_static::lazy_static;

fn get_string_from_raw_ptr(ptr: *const c_char) -> String {
     let c_str = unsafe {
        CStr::from_ptr(ptr)
    };
    let buf: &[u8] = c_str.to_bytes();
    let str_slice = std::str::from_utf8(buf).unwrap();
    str_slice.to_owned()
}

#[no_mangle]
pub unsafe extern "C" fn malloc_intercept(size: i32) {
    // if var_name.is_null() || var_type.is_null() {
    //     return
    // }

    // let mut prs = get_static_code_parser().lock().unwrap();
    // let mut prs = CodeParser::new();
    // prs.add_new_variable(String::from("Var"), 2, String::from("Type"));


    let mut mutguard = SHARED_MEMORY.value.lock().unwrap();
    *mutguard += 1;
    std::mem::drop(mutguard);
}

#[no_mangle]
pub unsafe extern "C" fn free_intercept() {
    // let prs = get_static_code_parser().lock().unwrap();
    // println!("Freed with value: {}", MYVAR.lock().unwrap());

    let mutguard = SHARED_MEMORY.value.lock().unwrap();
    let value: i32= mutguard.clone();
    std::mem::drop(mutguard);

    println!("Free called ref with {:?}", &value);
}




