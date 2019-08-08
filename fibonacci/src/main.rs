use std::io;
use std::str::FromStr;

mod fib;

fn main() {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        input = input.trim().to_string();

        // No input ends cycle and finishes program
        if input.is_empty() {
            break;
        }

        let mut n = i32::from_str(&input).expect("Error parsing i32 number");

        println!("Fibonacci is {}", fib::fib(n));
    }
}