extern crate image;
use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;

#[path = "math.rs"]
mod math;
extern crate num;
use num::Complex;
#[path = "bounds.rs"]
mod bounds;

/// Draws a rectangular portion of
/// the Mandelbrot set in a pixel buffer.
///
/// # Arguments
///
/// * `bounds` - The pair that defines the width and height of the buffer in pixels.
/// * `pixels` - The buffer in which each byte represents one halftone pixel.
/// * `upper_left` - The point on the complex area that describes the upper left corner of the buffer.
/// * `lower_right` - The point on the complex area that describes the lower right corner of the buffer.
///
pub fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = bounds::pixel_to_point(bounds, (column, row), upper_left, lower_right);
            // If the point belongs to the set...
            pixels[row * bounds.0 + column] = match math::escape_time(point, 255) {
                None => 0,                        // ... draw it as black pixel...
                Some(count) => 255 - count as u8, // ...otherwise, draw it as gray
            };
        }
    }
}

/// Writes an `pixels` buffer, whose dimensions are given
/// by the `bounds` argument, to a file named `filename`.
///
/// # Argumentsextern crate num;
///
/// * `bounds` - The pair that defines the width and height of the buffer in pixels.
/// * `pixels` - The buffer in which each byte represents one halftone pixel.
/// * `filename` - The output image file name.
///
pub fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    // The operator `?` specially introduced for the convenience of check.
    // Instead of explicitly writing:
    // let output = match File::create(filename) {
    //     Ok(f) => f,
    //     Err(e) => {
    //         return Err(e);
    //     }
    // };
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;
    Ok(())
}
