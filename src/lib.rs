extern crate pretend_std;

mod c_impl {
    #[no_mangle]
    pub extern "C" fn exported_function() -> libc::c_int {
        42
    }
}

pub use pretend_std::value;
