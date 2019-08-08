// #![allow(dead_code)]

extern crate num;
use num::Complex;

/// Attempts to determine whether `c` belongs to
/// the Mandelbrot set by limiting` limit` to iterations.
/// If `c` does not belong to the set, return `Some (i)`,
/// where `i` is the number of iterations required for `c`
/// to leave the circle of radius 2 centered at the origin.
/// If `c` can belong to a set (more precisely, if after
/// the limit of iterations it was not possible to prove
/// that `c` is not a set), then return `None`.
///
/// # Arguments
///
/// * `c` - Complex number.
/// * `limit` - Iteration constraints.
///
/// # Examples
///
/// ```
/// let mut number = 12;
/// let mut limit = 12;
/// let out_point = escape_time(number, limit);
///
/// ```
pub fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}
