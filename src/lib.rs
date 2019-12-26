use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uint};

#[no_mangle]
pub extern "C" fn print_hello_from_rust() {
    println!("Hello from Rust");
}

#[no_mangle]
pub extern "C" fn hm_chars(s: *const c_char) -> c_uint {
    assert!(!s.is_null());
    let c_str = unsafe { CStr::from_ptr(s) };
    let r_str = c_str.to_str().unwrap();
    r_str.chars().count() as c_uint
}

#[no_mangle]
pub extern "C" fn batman_song(length: c_uint) -> *mut c_char {
    let mut song = String::from("boom ");
    song.extend(std::iter::repeat("nana ").take(length as usize));
    song.push_str("Batman! boom");
    let c_str_song = CString::new(song).unwrap();
    c_str_song.into_raw()
}

#[no_mangle]
pub extern "C" fn free_song(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            CString::from_raw(s);
        }
    }
}

#[no_mangle]
pub extern "C" fn sum_of_even(n: *const c_uint, len: c_uint) -> c_uint {
    assert!(!n.is_null());
    let numbers = unsafe { std::slice::from_raw_parts(n, len as usize) };
    let sum = numbers
        .iter()
        .filter(|&v| v % 2 == 0)
        .fold(0u32, |acc, &v| acc + v);
    sum as c_uint
}

#[repr(C)]
pub struct Tuple {
    x: c_uint,
    y: c_uint,
}

impl From<(u32, u32)> for Tuple {
    fn from(tup: (u32, u32)) -> Self {
        Tuple { x: tup.0, y: tup.1 }
    }
}

impl From<Tuple> for (u32, u32) {
    fn from(tup: Tuple) -> Self {
        (tup.x, tup.y)
    }
}

fn compute_tuple(tup: (u32, u32)) -> (u32, u32) {
    let (a, b) = tup;
    (b + 1, a - 1)
}

#[no_mangle]
pub extern "C" fn flip_things_around(tup: Tuple) -> Tuple {
    compute_tuple(tup.into()).into()
}

pub struct Database {
    data: HashMap<String, u32>,
}

impl Database {
    fn new() -> Database {
        Database {
            data: HashMap::new(),
        }
    }

    fn insert(&mut self) {
        for i in 0..100000 {
            let zip = format!("{:05}", i);
            self.data.insert(zip, i);
        }
    }

    fn get(&self, zip: &str) -> u32 {
        self.data.get(zip).cloned().unwrap_or(0)
    }
}

#[no_mangle]
pub extern "C" fn database_new() -> *mut Database {
    Box::into_raw(Box::new(Database::new()))
}

#[no_mangle]
pub extern "C" fn database_insert(ptr: *mut Database) {
    assert!(!ptr.is_null());
    let database = unsafe { &mut *ptr };
    database.insert();
}

#[no_mangle]
pub extern "C" fn database_query(ptr: *const Database, zip: *const c_char) -> c_uint {
    assert!(!ptr.is_null());
    assert!(!zip.is_null());
    let database = unsafe { &*ptr };
    let zip = unsafe { CStr::from_ptr(zip) };
    let zip_str = zip.to_str().unwrap();
    database.get(zip_str)
}

#[no_mangle]
pub extern "C" fn database_free(ptr: *mut Database) {
    if !ptr.is_null() {
        unsafe {
            Box::from_raw(ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn print_str(ptr: *const c_char) {
    unsafe {
        print_from_c(ptr);
    }
}

extern "C" {
    pub fn print_from_c(x: *const c_char);
}
