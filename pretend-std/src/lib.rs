#![no_std]

extern "C" {
    fn exported_function() -> libc::c_int;
}

pub fn value() -> i32 {
    unsafe { exported_function() }
}
