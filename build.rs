extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let bindings = bindgen::Builder::default().header("hawktracer/lib/include/hawktracer.h").clang_arg("-I./hawktracer/lib/include/").generate().expect("Unable to generate bindings");
    println!("Manifest dir: {:?}", manifest_dir);

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let target = env::var("TARGET").unwrap();
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut lib_dir = manifest_dir.clone();
    let mut dll_dir = manifest_dir.clone();
    dll_dir.push("msvc");
    dll_dir.push("x64");
    lib_dir.push("msvc");
    lib_dir.push("x64");
    println!("cargo:rustc-link-search=all={}", lib_dir.display());
    //panic!("dlldir {:?}", dll_dir);
    for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir")  {
        let entry_path = entry.expect("Invalid fs entry").path();
        let file_name_result = entry_path.file_name();
        let mut new_file_path = manifest_dir.clone();
        if let Some(file_name) = file_name_result {
            let file_name = file_name.to_str().unwrap();
            if file_name.ends_with(".dll") {
                new_file_path.push(file_name);
                std::fs::copy(&entry_path, new_file_path.as_path()).expect("Can't copy from DLL dir");
            }
        }
    }
}