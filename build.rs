fn main() {
    cxx_build::bridge("src/ffi.rs")
        .file("src/timetagger.cpp")
        .std("c++14")
        .flag("-I/usr/include/timetagger")
        .flag("-O3") // Optimize for speed
        .flag("-DNDEBUG") // Disable assertions
        .compile("TT-rs");

    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=src/timetagger.cpp");
    println!("cargo:rerun-if-changed=src/timetagger.h");
    println!("cargo:rustc-link-lib=TimeTagger"); // If your platform needs linking against stdc++
}
