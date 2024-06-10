use crate as simplesl;
use simplesl_macros::export;

#[export]
mod add_math {
    pub const MIN_INT: i64 = i64::MIN;
    pub const MAX_INT: i64 = i64::MAX;
    pub fn count_ones(int: i64) -> u32 {
        int.count_ones()
    }
    pub fn count_zeros(int: i64) -> u32 {
        int.count_zeros()
    }
    pub fn leading_zeroes(int: i64) -> u32 {
        int.leading_zeros()
    }
    pub fn trailing_zeroes(int: i64) -> u32 {
        int.trailing_zeros()
    }
    pub fn leading_ones(int: i64) -> u32 {
        int.leading_ones()
    }
    pub fn trailing_ones(int: i64) -> u32 {
        int.trailing_ones()
    }
    pub fn swap_bytes(int: i64) -> i64 {
        int.swap_bytes()
    }
    pub fn reverse_bits(int: i64) -> i64 {
        int.reverse_bits()
    }
}
