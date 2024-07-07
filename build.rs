use std::env;
fn main() {
    let target_os :String = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "windows" {
        println!("cargo:rustc-link-lib=./resources/res");
    } else {
        println!("")
    }
}