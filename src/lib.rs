// Main lib file for this entire project.
#[cfg(target_arch = "x86_64")]
unsafe extern "C" {
    pub fn add_two_i64(a: i64, b: i64) -> i64;
}

#[cfg(target_arch = "x86")]
extern "C" {
    pub fn add_two_i32(a: i32, b: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_64() {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            assert_eq!(add_two_i64(40, 2), 42);
        }
    }

    #[test]
    fn test_add_32() {
        #[cfg(target_arch = "x86")]
        unsafe {
            assert_eq!(add_two_i32(40, 2), 42);
        }
    }
}
