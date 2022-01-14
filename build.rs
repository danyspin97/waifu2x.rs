use cmake::Config;
use path_absolutize::Absolutize;
use std::env;
use std::fs::create_dir;
use std::path::PathBuf;

fn main() {
    let root_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let proj_dir = root_dir.absolutize().unwrap();
    let models_dir = proj_dir.join("waifu2x").join("models");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let waifu2x_dir = out_dir.join("waifu2x");
    create_dir(&waifu2x_dir).unwrap_or_default();
    let waifu2x = Config::new("waifu2x")
        .out_dir(waifu2x_dir)
        .static_crt(true)
        .profile("MinSizeRel")
        .env("VULKAN_SDK", env::var("DEP_NCNN_VULKAN_DIR").unwrap())
        .define("MSVC_STATIC", "ON")
        .define("INCLUDE_LIST", env::var("DEP_NCNN_INCLUDE").unwrap())
        .define("LINK_LIST", env::var("DEP_NCNN_LIBRARY").unwrap())
        .define(
            if cfg!(feature = "upconv7_outside_model") {
                "WAIFU2X_UPCONV7_OUTSIDE_MODEL"
            } else if cfg!(feature = "upconv7") {
                "WAIFU2X_UPCONV7_ONLY"
            } else if cfg!(feature = "noise_outside_model") {
                "WAIFU2X_NOISE_OUTSIDE_MODEL"
            } else if cfg!(feature = "noise") {
                "WAIFU2X_NOISE_ONLY"
            } else {
                "WAIFU2X_FULL"
            },
            "ON",
        )
        .build();
    println!("cargo:models={}", models_dir.display());
    println!(
        "cargo:rustc-link-search=native={}",
        waifu2x.join("lib").display()
    );
    println!(
        "cargo:rustc-link-lib=static:-bundle={}",
        "waifu2x-ncnn-vulkan"
    );
    println!(
        "cargo:rustc-link-search=native={}",
        env::var("DEP_NCNN_LIBRARY").unwrap()
    );
    println!("cargo:rustc-link-lib=static:-bundle={}", "ncnn");
    // println!(
    //     "cargo:rustc-link-lib=static:-bundle={}",
    //     "MachineIndependent"
    // );
    println!("cargo:rustc-link-lib=dylib={}", "glslang");
    println!("cargo:rustc-link-lib=dylib={}", "SPIRV");
    //println!("cargo:rustc-link-lib=static:-bundle={}", "SPIRV");
    //println!("cargo:rustc-link-lib=static:-bundle={}", "GenericCodeGen");
    //println!("cargo:rustc-link-lib=static:-bundle={}", "OSDependent");
    //println!("cargo:rustc-link-lib=static:-bundle={}", "OGLCompiler");
    println!(
        "cargo:rustc-link-search=native={}",
        env::var("DEP_NCNN_VULKAN_LIB").unwrap()
    );
    if cfg!(windows) {
        println!("cargo:rustc-link-lib=static={}", "vulkan-1");
    } else {
        println!("cargo:rustc-link-lib=dylib={}", "gomp");
        println!("cargo:rustc-link-lib=dylib={}", "stdc++");
        println!("cargo:rustc-link-lib=dylib={}", "vulkan");
    }
}
