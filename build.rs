// update build.rs file as:
extern crate cc;

fn main() {
    //println!("cargo:rustc-link-search=all=src"); // works like "rustc -L src ..."
    //println!("cargo:rustc-link-lib=dylib=doubler.o"); // works like "rustc -l doubler.o"

    cc::Build::new()
        .file("src/fix_store.c")
        .include("/usr/local/opt/openssl/include")
        .compile("libfixstore.a");
}
