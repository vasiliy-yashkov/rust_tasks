#![allow(dead_code)]
use std::str::FromStr;

extern crate num;
use num::Complex;

/// Parses the string `s` containing a pair of coordinates,
/// for example: `"720x480"` or `"4.2,1.5"`.
/// More precisely, the `s` should look like <left><sep><right>,
/// where <sep> is the character specified in the argument `separator`,
/// and <left> and <right> are strings that can be parsed by the `T::method from_str`.
/// If the source string `s` could be parsed, then function returns `Some <(x, y)>`, otherwise `None`.
///
/// # Arguments
///
/// * `s` - String containing a pair of coordinates separated by character.
/// * `separator` - The dividing character.
///
/// # Examples
///
/// ```
/// let res = parse_pair::<f64>("0.5x1.5",'x');
/// assert_eq!(parse_pair::<f64>(res, Some((0.5, 1.5)));
///
/// ```
pub fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// Parses a pair of floating-point numbers,
/// separated by a character, and returns it as a complex number.
pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("4.42,-1.7319"),
        Some(Complex {
            re: 4.42,
            im: -1.7319
        })
    );
    assert_eq!(parse_complex(",8.4861"), None);
}
