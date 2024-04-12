use std::ffi::{c_char, CStr};

use std::fmt::Debug;
use std::ptr;

use libc::IPC_CREAT;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TestStruct {
    pub value: i32,
    pub mallocs: i32,
    pub frees: i32,
}

/**
 * TODO: 
 *  - >>>> Maybe Make lazystatic for CodeParser
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



// Create a global mutable static i32 variable
pub static mut GLOBAL_VAR: i32 = 0;

fn get_string_from_raw_ptr(ptr: *const c_char) -> String {
     let c_str = unsafe {
        CStr::from_ptr(ptr)
    };
    let buf: &[u8] = c_str.to_bytes();
    let str_slice = std::str::from_utf8(buf).unwrap();
    str_slice.to_owned()
}


// TODO: Implement write to shmem
pub fn write_to_shmem<T>(data: T, shm_key: i32) where T: Copy + Debug {
    // Write data to shared memory with id shmem_id
    // Get the size of T
    let mem_size = std::mem::size_of::<T>() as libc::size_t;
    unsafe {
        // shmflg = 0 means shm already exists
        let shmem_id = libc::shmget(shm_key, mem_size, 0o777 | IPC_CREAT);
        let ptr = libc::shmat(shmem_id, ptr::null() as *const libc::c_void, 0) as *mut T;
        println!("Write to shmem {:?}", ptr);
        if ptr.is_null() || (ptr as isize) == -1 {
            panic!("Failed to attach to shmem on write");
        }
        ptr::write(ptr, data);
        libc::shmdt(ptr as *const libc::c_void);
    }
}

pub fn write_to_new_shmem<T>(data: T, key: i32) -> i32 where T: Copy{
    let mem_size = std::mem::size_of::<T>() as libc::size_t;
    let shm_id = unsafe { libc::shmget(key, mem_size, libc::IPC_CREAT | libc::IPC_EXCL | 0o777) };
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
        println!("Write to shmem {:?}", ptr);
        libc::shmdt(ptr as *const libc::c_void);
    }
    shm_id
}


// TODO: Return a value on error instead of panic
pub fn read_from_shmem<T>(shm_key: i32) -> T where T: Copy + Debug {
    let mem_size = std::mem::size_of::<T>() as libc::size_t;
    println!("Read shmid");
    let shmem_id = unsafe {
        libc::shmget(shm_key, mem_size, 0o777 | IPC_CREAT)
    };
    println!("Start read");
    let ptr = unsafe { libc::shmat(shmem_id,ptr::null() as *const libc::c_void, 0) } as *mut T;
    println!("Read from shmem {:?}", ptr);
    if ptr.is_null() || (ptr as isize) == -1 {
        panic!("Failed to attach to shmem on read");
    }
    let data = unsafe { *ptr };
    println!("Data {:?}", data);
    unsafe {
        libc::shmdt(ptr as *const libc::c_void);
        println!("shmdt Data");
        libc::shmctl(shmem_id, libc::IPC_RMID, ptr::null_mut());
        println!("shmctl Data");
    }
    data
}


#[no_mangle]
pub unsafe extern "C" fn malloc_intercept(size: i32) {
    let shm_key= 43;
    let mut tst_struct = read_from_shmem::<TestStruct>(shm_key);
    // TODO: Check if shmem already open to avoid recursive malloc calls
    // Or open shmem at the beginning of C program and write at the end
    // But with this we need to add a new argument (pointer to TestStruct)
    tst_struct.mallocs += 1;

    write_to_shmem(tst_struct, shm_key);
}

#[no_mangle]
pub 
unsafe extern "C" fn free_intercept() {
    let shm_key = 43;
    let mut tst_struct = read_from_shmem::<TestStruct>(shm_key);

    tst_struct.frees += 1;

    write_to_shmem(tst_struct, shm_key);
}
