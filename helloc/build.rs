// build.rs
// Bring in a dependency on an externally maintained
// `cc` package which manages invoking the C compiler
use std::env;

extern crate cc;


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=hello");
}

