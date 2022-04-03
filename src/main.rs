fn main() {
    // Uncomment this line to fix linking.
    // Related to https://github.com/rust-lang/rust/issues/47384
    // reproduce_used_linker_function::c_impl::init();

    println!("output: {}", reproduce_used_linker_function::value());
}
