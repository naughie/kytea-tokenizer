#[cfg(feature = "ffi")]
fn main() {
    println!("cargo:rerun-if-changed=lib/ckytea.cpp");
    println!("cargo:rustc-link-lib=kytea");
    cc::Build::new()
        .cpp(true)
        .file("lib/ckytea.cpp")
        .flag("-Wno-deprecated")
        .cpp_link_stdlib("stdc++")
        .compile("libckytea.a");
}

#[cfg(not(feature = "ffi"))]
fn main() {}
