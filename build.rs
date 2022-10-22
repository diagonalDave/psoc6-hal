use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let host_triple = env::var("HOST").unwrap();

    if host_triple == target {
        println!("cargo:rustc-cfg=native");
    }
    if target.starts_with("thumbv6m-") {
        println!("cargo:rustc-cfg=armv6m");
        //when compiling for the CM4 core use a different device.x
    } else if target.starts_with("thumbv7em-") {
        println!("cargo:rustc-cfg=armv7em");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
