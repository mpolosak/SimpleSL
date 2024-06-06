use crate as simplesl;
use simplesl_macros::export;

#[export]
mod add_math {
    const MIN_INT: i64 = i64::MIN;
    const MAX_INT: i64 = i64::MAX;
    fn count_ones(int: i64) -> u32 {
        int.count_ones()
    }
    fn count_zeros(int: i64) -> u32 {
        int.count_zeros()
    }
    fn leading_zeroes(int: i64) -> u32 {
        int.leading_zeros()
    }
    fn trailing_zeroes(int: i64) -> u32 {
        int.trailing_zeros()
    }
    fn leading_ones(int: i64) -> u32 {
        int.leading_ones()
    }
    fn trailing_ones(int: i64) -> u32 {
        int.trailing_ones()
    }
    fn swap_bytes(int: i64) -> i64 {
        int.swap_bytes()
    }
    fn reverse_bits(int: i64) -> i64 {
        int.reverse_bits()
    }
}
