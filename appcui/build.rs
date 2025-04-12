use std::env;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(wasm_windows)");
    println!("cargo::rustc-check-cfg=cfg(wasm_unix)");

    if env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        println!("cargo:rustc-link-lib=user32");
    }

    if env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default() == "wasm32" || env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "emscripten" {
        if env::var("HOST").unwrap_or_default().contains("windows") {
            println!("cargo:rustc-cfg=wasm_windows");
        } else {
            println!("cargo:rustc-cfg=wasm_unix");
        }
    }
}
