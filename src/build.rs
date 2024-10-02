fn main() {
    // Tell cargo to tell rustc to link the X11 library
    println!("cargo:rustc-link-lib=X11");
}