use std::ffi::{c_char, CStr};

mod parser;

use std::ptr;

/**
 * TODO: 
 *  - Make lazystatic for CodeParser
 *  - Add appropriate values to CodeParser on malloc intercept
 *  - Intercept vulnerable functions (strcpy, strcat, ...)
 *  - Remove variable from CodeParser on free intercept
 *  - Might also check data leaks
 *  - Create Shmem on C program start
 *  - Write to Shmem on C program end
 *  - Read Shmem from Rust 
 *  - Check vulns on overriden functions (strcpy, strcat, ...)
 *  - Maybe some rust functions need to extern "C" to be used in C
 */



fn get_string_from_raw_ptr(ptr: *const c_char) -> String {
     let c_str = unsafe {
        CStr::from_ptr(ptr)
    };
    let buf: &[u8] = c_str.to_bytes();
    let str_slice = std::str::from_utf8(buf).unwrap();
    str_slice.to_owned()
}

pub fn write_to_shared_memory<T>(data: T) -> i32 where T: Copy{
    let key = 69420;
    let mem_size = std::mem::size_of::<T>() as libc::size_t;
    let shm_id = unsafe { libc::shmget(key, mem_size, libc::IPC_CREAT | 0o666) };
    println!("get shmem id {}", shm_id);
    if shm_id < 0 {
        panic!("Failed to write to shmem");
    }
    let ptr = unsafe { libc::shmat(shm_id, ptr::null() as *const libc::c_void, 0) as *mut T};
    println!("attach shmem {:?}", ptr);
    if (ptr as isize) == -1{
        panic!("Failed to attach to shmem {}", std::io::Error::last_os_error());
    }

    unsafe {
        ptr::write(ptr, data);
        println!("Write to shmem");
        libc::shmdt(ptr as *const libc::c_void);
    }
    shm_id
}

pub fn read_from_shmem<T>(shm_id: i32) -> T where T: Copy {
    let ptr = unsafe { libc::shmat(shm_id, ptr::null(), 0) } as *mut T;
    if ptr.is_null() {
        panic!("Failed to attach to shmem");
    }
    let data = unsafe { *ptr };
    unsafe {
        libc::shmdt(ptr as *const _ as *mut libc::c_void);
        libc::shmctl(shm_id, libc::IPC_RMID, ptr::null_mut());
    }
    data
}


#[no_mangle]
pub unsafe extern "C" fn malloc_intercept(size: i32) {
    

}

#[no_mangle]
pub unsafe extern "C" fn free_intercept() {
}




