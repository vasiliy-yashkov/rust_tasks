/// Function returns n's Fibonacci number i64
///
/// # Arguments
///
/// * `n` - The single i32 argument n.
///
/// # Examples
///
/// ```
/// let mut n = 6;
/// let answer = fib(n);
///
/// assert_eq!(8, answer);
/// ```
pub fn fib(n: i32) -> i64 {
    assert!(n <= 90);
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut num1: i64 = 0;
    let mut num2: i64 = 1;

    for _x in 2..n {
        let res = num1 + num2;
        num1 = num2;
        num2 = res;
    }
    return num1 + num2;
}

#[test]
fn test_fib() {
    assert_eq!(fib(6), 8);
    assert_eq!(fib(9), 34);
}
