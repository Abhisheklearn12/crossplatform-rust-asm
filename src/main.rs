// Added this to see the output on screen as a support only, that's it and I can add more targets as well.
// I target here mainly:
// ==> X_86 (64 bit, 32 bit) ==> for unix and windows GNU/CLANG.
fn main() {
    println!("Testing kvstore_simd library...");

    // 64-bit x86 (Unix + Windows GNU)
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let a: i64 = 40;
        let b: i64 = 2;
        let result = kvstore::add_two_i64(a, b);
        println!("add_two_i64({}, {}) = {}", a, b, result);
    }

    // 32-bit x86 (Unix + Windows GNU)
    #[cfg(target_arch = "x86")]
    unsafe {
        let a: i32 = 40;
        let b: i32 = 2;
        let result = kvstore::add_two_i32(a, b);
        println!("add_two_i32({}, {}) = {}", a, b, result);
    }

    // Optionally, you can add tests for ARM later if you implement NEON intrinsics
    // #[cfg(target_arch = "arm")]
    // unsafe { ... }
    // #[cfg(target_arch = "aarch64")]
    // unsafe { ... }

    println!("All tests executed successfully!");
}
