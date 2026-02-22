use std::path::PathBuf;

fn main() {
    let header = "../wrapper.h";

    let bindings = bindgen::Builder::default()
        .header(header)
        // Don't bind libc types
        .blocklist_item("_.*")
        .blocklist_item("FILE")
        .blocklist_item("time_t")
        // Don't bint functions which use libc types
        .blocklist_item("ZkRead_newFile")
        .blocklist_item("ZkWrite_newFile")
        .allowlist_item(".*Zk.*")
        // Fix exceptions
        .raw_line("#![allow(deprecated)]")
        .raw_line("use std::os::unix::raw::time_t;")
        // Improve enums
        .rustified_enum(".*")
        // Add Includes
        .clang_arg("-I../vendor/ZenKitCAPI/include")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("../src/bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");

    println!("Bindings successfully generated at src/bindings.rs");
}
