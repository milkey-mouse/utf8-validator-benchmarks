use std::{env, path::PathBuf};

fn get_cpp_link_stdlib() -> Option<String> {
    if let Ok(stdlib) = env::var("CXXSTDLIB") {
        if stdlib.is_empty() {
            None
        } else {
            Some(stdlib)
        }
    } else {
        let target = env::var("TARGET").unwrap();
        if target.contains("msvc") {
            None
        } else if target.contains("apple") {
            Some("c++".to_string())
        } else if target.contains("freebsd") {
            Some("c++".to_string())
        } else if target.contains("openbsd") {
            Some("c++".to_string())
        } else {
            Some("stdc++".to_string())
        }
    }
}

fn main() {
    let dst = cmake::Config::new("../simdjson")
        .define("SIMDJSON_JUST_LIBRARY", "ON")
        .define("SIMDJSON_BUILD_STATIC", "ON")
        .uses_cxx11()
        .build();
        
    println!("cargo:rustc-link-search=native={}/lib64", dst.display());
    println!("cargo:rustc-link-lib=static=simdjson");
    
    if let Some(stdlib) = get_cpp_link_stdlib() {
        println!("cargo:rustc-link-lib={}", stdlib);
    }

    eprintln!("bindings");
    let bindings = bindgen::Builder::default()
        .clang_args(&["-xc++", "-std=c++17", "-I../simdjson/include"])
        .header("../simdjson/include/simdjson/implementation.h")
        .whitelist_function("simdjson::validate_utf8")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("unable to generate bindings");
    eprintln!("bindings done");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
