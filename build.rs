extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;
use std::fs;

//In order to build 64 bit hawktracer:
// mkdir build
// cd build
// cmake .. -G "Visual Studio 15 2017 Win64" -T v141,host=x64
// cmake --build .
fn main() {
    #[cfg(feature = "bindgen")] {
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
    }

    #[cfg(not(feature = "bindgen"))] {
        copy_pregenerated_bindings();
    }

    build_project();

    let target = env::var("TARGET").unwrap();
    if target.contains("pc-windows") {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()); 
        let mut build_output_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        build_output_path.push("build");
        build_output_path.push("lib");
        
        #[cfg(debug_assertions)] { 
            build_output_path.push("Debug");
        }

        #[cfg(not(debug_assertions))] {
            build_output_path.push("Release");
        }

        println!("cargo:rustc-link-search=all={}", build_output_path.display());
        println!("cargo:rustc-link-lib=static=hawktracer");
    }
}

fn build_project() {
    cmake::Config::new("hawktracer")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("BUILD_STATIC_LIB", "ON")
        .generator("Visual Studio 15 2017 Win64")
        .build_target("hawktracer")
        .build();
}


#[cfg(not(feature = "bindgen"))]
fn copy_pregenerated_bindings() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    fs::copy(crate_path.join("pregenerated_bindings.rs"), out_path.join("bindings.rs"))
        .expect("Couldn't find pregenerated bindings!");
}
