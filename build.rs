use std::{ffi::OsString, io::Write, path::Path};

fn get_lib_name() -> &'static str {
    #[cfg(target_arch = "x86_64")]
    return "GFSDK_Aftermath_Lib.x64";

    #[cfg(target_arch = "x86")]
    return "GFSDK_Aftermath_Lib.x86";
    #[allow(unreachable_code)]
    {
        panic!()
    }
}

fn main() {
    println!("cargo:rustc-link-lib={}", get_lib_name());
    println!("cargo:rustc-link-lib={}", get_lib_name());
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = Path::new(&dir).join("lib\\nsight-aftermath\\lib\\x64\\");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
}
