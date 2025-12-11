use crate as simplesl;
use simplesl_macros::export;

#[export(Math)]
mod inner {
    use std::f64::consts;

    pub const MIN_INT: i64 = i64::MIN;
    pub const MAX_INT: i64 = i64::MAX;
    pub const E: f64 = consts::E;
    pub const PI: f64 = consts::PI;
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

    pub fn ilog(num: i64, base: i64) -> Option<u32> {
        num.checked_ilog(base)
    }

    pub fn ilog2(num: i64) -> Option<u32> {
        num.checked_ilog2()
    }

    pub fn ilog10(num: i64) -> Option<u32> {
        num.checked_ilog10()
    }

    pub fn floor(num: f64) -> f64 {
        num.floor()
    }

    pub fn ceil(num: f64) -> f64 {
        num.ceil()
    }

    pub fn round(num: f64) -> f64 {
        num.round()
    }

    pub fn round_ties_even(num: f64) -> f64 {
        num.round_ties_even()
    }

    pub fn trunc(num: f64) -> f64 {
        num.trunc()
    }

    pub fn fract(num: f64) -> f64 {
        num.fract()
    }

    pub fn ln(num: f64) -> f64 {
        num.ln()
    }

    pub fn log(num: f64, base: f64) -> f64 {
        num.log(base)
    }

    pub fn log2(num: f64) -> f64 {
        num.log2()
    }

    pub fn log10(num: f64) -> f64 {
        num.log10()
    }

    pub fn sin(angle: f64) -> f64 {
        angle.sin()
    }

    pub fn cos(angle: f64) -> f64 {
        angle.cos()
    }

    pub fn tan(angle: f64) -> f64 {
        angle.tan()
    }

    pub fn asin(num: f64) -> f64 {
        num.asin()
    }

    pub fn acos(num: f64) -> f64 {
        num.acos()
    }

    pub fn atan(num: f64) -> f64 {
        num.atan()
    }

    pub fn atan2(num1: f64, num2: f64) -> f64 {
        num1.atan2(num2)
    }

    pub fn exp_m1(num: f64) -> f64 {
        num.exp_m1()
    }

    pub fn ln_1p(num: f64) -> f64 {
        num.ln_1p()
    }

    pub fn sinh(num: f64) -> f64 {
        num.sinh()
    }

    pub fn cosh(num: f64) -> f64 {
        num.cosh()
    }

    pub fn tanh(num: f64) -> f64 {
        num.tanh()
    }

    pub fn asinh(num: f64) -> f64 {
        num.asinh()
    }

    pub fn acosh(num: f64) -> f64 {
        num.acosh()
    }

    pub fn atanh(num: f64) -> f64 {
        num.atanh()
    }

    pub fn is_nan(num: f64) -> bool {
        num.is_nan()
    }

    pub fn is_infinite(num: f64) -> bool {
        num.is_infinite()
    }

    pub fn is_finite(num: f64) -> bool {
        num.is_finite()
    }

    pub fn is_normal(num: f64) -> bool {
        num.is_normal()
    }

    pub fn is_subnormal(num: f64) -> bool {
        num.is_subnormal()
    }

    pub fn is_sign_positive(num: f64) -> bool {
        num.is_sign_positive()
    }

    pub fn is_sign_negative(num: f64) -> bool {
        num.is_sign_negative()
    }

    pub fn to_bits(num: f64) -> i64 {
        num.to_bits() as i64
    }

    pub fn from_bits(num: i64) -> f64 {
        f64::from_bits(num as u64)
    }
}
