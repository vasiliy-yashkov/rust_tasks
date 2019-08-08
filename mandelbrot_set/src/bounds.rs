extern crate num;
use num::Complex;

/// Knowing a row and column pixel of the output image,
/// returns the corresponding point on a complex plane.
///
///
/// The parameters `upper_left` and `lower_right` are points
/// on the complex plane that describe the area covered by the image.
/// # Arguments
///
/// * `bounds` - The pair that defines the width and height of the image in pixels.
/// * `pixel` - The pair (row, column) defining a specific pixel of the image.
/// * `upper_left` - The point on the complex area that describes the upper left corner of a area.
/// * `lower_right` - The point on the complex area that describes the lower right corner of a area.
///
/// # Examples
///
/// ```
/// let res = pixel_to_point((100, 100),
///            (25, 75),
///            Complex { re: -1.0, im: 1.0 },
///            Complex { re: 1.0, im: -1.0 }
///        );
/// assert_eq!(res, Complex { re: -0.5, im: -0.5 });
///
/// ```
pub fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        // Explicit type conversion
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        // Why is subtraction here? pixel.1 increases when moving down,
        // while the imaginary part increases when moving up.
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 100),
            (25, 75),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: -0.5 }
    );
}
