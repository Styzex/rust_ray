fn main() {
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=dylib=GL");
        println!("cargo:rustc-link-lib=dylib=GLU");
    }
}
