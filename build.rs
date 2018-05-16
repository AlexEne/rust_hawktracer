extern crate bindgen;

use std::env;
use std::path::PathBuf;

//In order to build 64 bit hawktracer:
// mkdir build
// cd build
// cmake .. -G "Visual Studio 15 2017 Win64" -T v141,host=x64
// cmake --build .
fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let bindings = bindgen::Builder::default()
        .header("hawktracer/lib/include/hawktracer.h")
        .clang_arg("-I./hawktracer/lib/include/")
        .clang_arg("-I./hawktracer/build/lib/include/")
        .generate()
        .expect("Unable to generate bindings");
    println!("Manifest dir: {:?}", manifest_dir);

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let target = env::var("TARGET").unwrap();
    if target.contains("pc-windows") {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let mut lib_dir = manifest_dir.clone();
        let mut dll_dir = manifest_dir.clone();
        if target.contains("msvc") {
            lib_dir.push("msvc");
            dll_dir.push("msvc");
        } else {
            lib_dir.push("gnu-mingw");
            dll_dir.push("gnu-mingw");
        }
        if target.contains("x86_64") {
            lib_dir.push("x64");
            dll_dir.push("x64");
        } else {
            lib_dir.push("x86");
            dll_dir.push("x86");
        }
        println!("cargo:rustc-link-search=all={}", lib_dir.display());
        println!("cargo:rustc-link-lib=dylib=hawktracer");
        //println!("cargo:rustc-link-search=native=/msvc/x64");
        for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir") {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path = manifest_dir.clone();
            if let Some(file_name) = file_name_result {
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".dll") {
                    new_file_path.push(file_name);
                    std::fs::copy(&entry_path, new_file_path.as_path())
                        .expect("Can't copy from DLL dir");
                }
            }
        }
    }
}
