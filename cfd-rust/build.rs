extern crate cmake;
use cmake::Config;

fn main()
{
    let dst = Config::new("cfd-cmake")
        .define("ENABLE_TESTS", "off")
        .define("ENABLE_JS_WRAPPER", "off")
        .define("ENABLE_CAPI", "on")
        .define("ENABLE_SHARED", "on")
        .build();

    println!("cargo:rustc-link-search=native={}/build/Debug", dst.display());
    println!("cargo:rustc-link-lib=cfd");
    println!("cargo:rustc-link-lib=cfdcore");
    println!("cargo:rustc-link-lib=wally");
}
