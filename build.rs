use std::env;
use std::path::PathBuf;

fn main() {
    println!(r"cargo:rustc-link-search=\DirectXTex\bin\CMake");

    let bindings = bindgen::Builder::default()
        .header(r"DirectXTex\DirectXTex\DirectXTex.h")
        .disable_name_namespacing()
        .opaque_type("*")
        .whitelist_function("DirectX::IsCompressed")
        .clang_args(&["-x", "c++"])
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings");
}
