
use std::env;

fn target() -> String {
    env::var("TARGET").unwrap()
}

fn main() {
    let mut build = cc::Build::new();
    let target = target();

    if target.contains("x86_64") {
        if target.contains("windows-gnu") {
            build.file("c/simd_x86_64_windows_gnu.S");
        } else {
            build.file("c/simd_x86_64_unix.S");
        }
    }

    if target.contains("i686") || target.contains("i386") {
        if target.contains("windows-gnu") {
            build.file("c/simd_x86_32_windows_gnu.S");
        } else {
            build.file("c/simd_x86_32_unix.S");
        }
    }

    build.compile("kvstore_simd");
}
