use crate::{Float64, Radian64};

use super::{log, ln1p, sqrt};

consts!{
const LN2: Float64 = 0.693147180559945309417232121458176568; /* 0x3fe62e42,  0xfefa39ef*/
}

/* asinh(x) = sign(x)*log(|x|+sqrt(x*x+1)) ~= x - x^3/6 + o(x^5) */
/// Inverse hyperbolic sine
///
/// Calculates the inverse hyperbolic sine of `x`.
/// Is defined as `sgn(x)*log(|x|+sqrt(x*x+1))`.
#[export_name = "__l2math_asinh"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn asinh(mut x: Float64) -> Radian64 {
    let mut u = x.to_bits();
    let e = ((u >> 52) as usize) & 0x7ff;
    let sign = (u >> 63) != 0;

    /* |x| */
    u &= (!0) >> 1;
    x = Float64::from_bits(u);

    if e >= 0x3ff + 26 {
        /* |x| >= 0x1p26 or inf or nan */
        x = log(x) + LN2;
    } else if e >= 0x3ff + 1 {
        /* |x| >= 2 */
        x = log(2.0 * x + 1.0 / (sqrt(x * x + 1.0) + x));
    } else if e >= 0x3ff - 26 {
        /* |x| >= 0x1p-26, up to 1.6ulp error in [0.125,0.5] */
        x = ln1p(x + x * x / (sqrt(x * x + 1.0) + 1.0));
    } else {
        /* |x| < 0x1p-26, raise inexact if x != 0 */
        let x1p120 = Float64::from_bits(0x4770000000000000);
        force_eval!(x + x1p120);
    }

    if sign {
        -x
    } else {
        x
    }
}
