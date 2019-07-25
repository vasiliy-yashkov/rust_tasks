use std::io::Write; // For writing values to output stream
use std::str::FromStr; // For getting values from string 

/// Function to find greatest common divisor 
/// between two unsigned numbers
/// 
/// # Arguments
///
/// * `n` - First unsigned number.
/// * `m` - Second unsigned number.
/// 
/// # Examples
///
/// ```
/// let mut n = 12, mut m = 18;
/// let answer = gcd(n, m);
///
/// assert_eq!(6, answer);
/// ```
fn gcd(mut n: u64, mut m: u64) -> u64 {
    debug_assert!(n != 0 && m != 0); // Check for null values
    println!("Find divisor between {} and {}", n, m); // Print info
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    return n; // Can be use n without 'return' keyword and ';'
}

fn main() {
    let mut res = gcd(12, 124);

    println!("{}", res);

    res = gcd(50, 8880);

    println!("{}", res);

    res = gcd(12, 18);

    println!("{}", res);

    gcd_from_args();
}

fn gcd_from_args() {
    let mut vec: Vec<u64> = Vec::new();
    for arg in std::env::args().skip(1) {
        vec.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if vec.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = vec[0];

    for m in &vec[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", vec, d);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
