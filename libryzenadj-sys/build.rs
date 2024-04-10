use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Remove CMAKE_INTERPROCEDURAL_OPTIMIZATION option to prevent linking errors
    Command::new("sed").args(["-i", "s/set(CMAKE_INTERPROCEDURAL_OPTIMIZATION TRUE)/set(CMAKE_INTERPROCEDURAL_OPTIMIZATION FALSE)/g", "./RyzenAdj/CMakeLists.txt"])
        .status()
        .expect("Failed to update CMakeLists.txt");

    if env::var("DOCS_RS").unwrap_or_else(|_| "0".to_string()) == "0" {
        let dst = cmake::Config::new("RyzenAdj")
            .define("BUILD_SHARED_LIBS", "OFF")
            .profile("Release")
            .build_target("libryzenadj")
            .build();
        //panic!("dst: {:?}", dst.display());
        println!("cargo:rustc-link-search=native={}/build", dst.display());
        println!("cargo:rustc-link-lib=static=ryzenadj");
        println!("cargo:rustc-link-lib=dylib=pci");
    }
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        //.default_enum_style(EnumVariation::NewType { is_bitfield: false })
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
