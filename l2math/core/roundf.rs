use crate::Float32;

use super::copysignf;
use super::truncf;

/// Rounds `x` to the nearest integer in the direction of the current rounding mode.
#[inline]
#[export_name = "__l2math_roundf"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn roundf(x: Float32) -> Float32 {
    truncf(x + copysignf(0.5 - 0.25 * Float32::EPSILON, x))
}

// PowerPC tests are failing on LLVM 13: https://github.com/rust-lang/rust/issues/88520
#[cfg(not(target_arch = "powerpc64"))]
#[cfg(test)]
mod tests {
    use super::roundf;

    #[test]
    fn negative_zero() {
        assert_eq!(roundf(-0.0_f32).to_bits(), (-0.0_f32).to_bits());
    }

    #[test]
    fn sanity_check() {
        assert_eq!(roundf(-1.0), -1.0);
        assert_eq!(roundf(2.8), 3.0);
        assert_eq!(roundf(-0.5), -1.0);
        assert_eq!(roundf(0.5), 1.0);
        assert_eq!(roundf(-1.5), -2.0);
        assert_eq!(roundf(1.5), 2.0);
    }
}
