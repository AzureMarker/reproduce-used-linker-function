extern crate pretend_std;

pub mod c_impl {
    /// Call this somewhere to force Rust to link this module.
    /// The call doesn't need to execute, just exist.
    ///
    /// See https://github.com/rust-lang/rust/issues/47384
    pub fn init() {}

    #[no_mangle]
    pub extern "C" fn exported_function() -> libc::c_int {
        42
    }
}

pub use pretend_std::value;
