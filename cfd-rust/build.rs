extern crate cmake;
use cmake::Config;
use std::env;

fn main()
{
    // cargo build -vv
    let _build_type = env::var("PROFILE").unwrap();
    let release_name = "release";

    let release_type = "Release";
    let debug_type = "Debug";
    let cmake_build_type = if _build_type == release_name {
        release_type
    } else {
        debug_type
    };

    let separator_slash = "/";
    let separator_back = "\\";
    let _os_type = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let os_windows = "windows";
    let separator = if _os_type == os_windows {
        separator_back
    } else {
        separator_slash
    };

    let dst = Config::new("cfd-cmake")
        .define("ENABLE_TESTS", "off")
        .define("ENABLE_JS_WRAPPER", "off")
        .define("ENABLE_CAPI", "on")
        .define("ENABLE_SHARED", "on")
        .define("CMAKE_BUILD_TYPE", cmake_build_type)
        .build();

    println!("cargo:rustc-link-search=native={}{}build{}{}", dst.display(), separator, separator, cmake_build_type);
    println!("cargo:rustc-link-lib=cfd");
    println!("cargo:rustc-link-lib=cfdcore");
    println!("cargo:rustc-link-lib=wally");
}
