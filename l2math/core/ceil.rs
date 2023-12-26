#![allow(unreachable_code)]

use crate::Float64;

const TOINT: Float64 = 1. / Float64::EPSILON;

/// Ceil
///
/// Finds the nearest integer greater than or equal to `x`.
#[export_name = "__l2math_ceil"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn ceil(x: Float64) -> Float64 {
    // On wasm32 we know that LLVM's intrinsic will compile to an optimized
    // `Float64.ceil` native instruction, so we can leverage this for both code size
    // and speed.
    llvm_intrinsically_optimized! {
        #[cfg(target_arch = "wasm32")] {
            return unsafe { ::core::intrinsics::ceilf64(x) }
        }
    }
    #[cfg(all(target_arch = "x86", not(target_feature = "sse2")))]
    {
        //use an alternative implementation on x86, because the
        //main implementation fails with the x87 FPU used by
        //debian i386, probablly due to excess precision issues.
        //basic implementation taken from https://github.com/rust-lang/libm/issues/219
        use super::fabs;
        if fabs(x).to_bits() < 4503599627370496.0_f64.to_bits() {
            let truncated = x as i64 as Float64;
            if truncated < x {
                return truncated + 1.0;
            } else {
                return truncated;
            }
        } else {
            return x;
        }
    }
    let u: u64 = x.to_bits();
    let e: i64 = (u >> 52 & 0x7ff) as i64;

    if e >= 0x3ff + 52 || x == 0. {
        return x;
    }
    // y = int(x) - x, where int(x) is an integer neighbor of x
    let y = if (u >> 63) != 0 {
        x - TOINT + TOINT - x
    } else {
        x + TOINT - TOINT - x
    };
    // special case because of non-nearest rounding modes
    if e < 0x3ff {
        force_eval!(y);
        return if (u >> 63) != 0 { -0. } else { 1. };
    }
    if y < 0. {
        x + y + 1.
    } else {
        x + y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f64::*;

    #[test]
    fn sanity_check() {
        assert_eq!(ceil(1.1), 2.0);
        assert_eq!(ceil(2.9), 3.0);
    }

    /// The spec: https://en.cppreference.com/w/cpp/numeric/math/ceil
    #[test]
    fn spec_tests() {
        // Not Asserted: that the current rounding mode has no effect.
        assert!(ceil(NAN).is_nan());
        for f in [0.0, -0.0, INFINITY, NEG_INFINITY].iter().copied() {
            assert_eq!(ceil(f), f);
        }
    }
}
