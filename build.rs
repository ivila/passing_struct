fn main() {
    let mut build = cc::Build::new();

    build.file("./c_src/print_memory.c").compile("print_memory");

    println!("cargo:rustc-link-lib=static=print_memory");
}
