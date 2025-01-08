fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/timetagger.cpp")
        .flag_if_supported("-std=c++14")
        .flag_if_supported("/MD")  // Multithreaded DLL runtime
        .flag_if_supported("/O2")  // Optimize for speed
        .flag_if_supported("/DNDEBUG")  // Define NDEBUG for release builds
        .compile("TT-rs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/timetagger.cpp");
    println!("cargo:rerun-if-changed=src/timetagger.h");
    println!("cargo:rustc-link-lib=TimeTagger"); // If your platform needs linking against stdc++

}
