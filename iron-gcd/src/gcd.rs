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
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
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