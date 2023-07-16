// Generate the nth Fibonacci number.
// https://en.wikipedia.org/wiki/Fibonacci_sequence
fn main() {
    let n = 5;
    let f = fibonacci(n);
    println!("The {n}th Fibonacci number is {f}.");

    let n = 10;
    let f = fibonacci(n);
    println!("The {n}th Fibonacci number is {f}.");

    let n = 50;
    let f = fibonacci(n);
    println!("The {n}th Fibonacci number is {f}.");
}

#[test]
fn generate_fibonacci_numbers_iteratively() {
    let n = 0;
    let f = fibonacci(n);
    assert_eq!(f, 0);

    let n = 1;
    let f = fibonacci(n);
    assert_eq!(f, 1);

    let n = 2;
    let f = fibonacci(n);
    assert_eq!(f, 1);

    let n = 5;
    let f = fibonacci(n);
    assert_eq!(f, 5);

    let n = 10;
    let f = fibonacci(n);
    assert_eq!(f, 55);

    let n = 15;
    let f = fibonacci(n);
    assert_eq!(f, 610);

    let n = 19;
    let f = fibonacci(n);
    assert_eq!(f, 4181);

    let n = 50;
    let f = fibonacci(n);
    assert_eq!(f, 12586269025);

    let n = 100;
    let f = fibonacci(n);
    assert_eq!(f, 354224848179261915075);
}

// Iterative algorithm
fn fibonacci(n: u128) -> u128 {
    if n < 2 {
        return n;
    }

    let mut previous_fib = 1;
    let mut current_fib = 1;
    for _ in 2..n {
        // Compute the next number and remember the previous number
        // We use a destructuring assignment here to save a temp variable.
        (previous_fib, current_fib) = (current_fib, current_fib + previous_fib)
    }

    current_fib
}

// Recursive algorithm
// This is very ineffective without optimizations like memoization.
// Expect this to run for a very long time if n > 50.
#[allow(dead_code)]
fn recursive_fibonacci(n: u128) -> u128 {
    if n < 2 {
        return n;
    }

    recursive_fibonacci(n - 1) + recursive_fibonacci(n - 2)
}

#[test]
fn generate_fibonacci_numbers_recursively() {
    let n = 0;
    let f = recursive_fibonacci(n);
    assert_eq!(f, 0);

    let n = 1;
    let f = recursive_fibonacci(n);
    assert_eq!(f, 1);

    let n = 2;
    let f = recursive_fibonacci(n);
    assert_eq!(f, 1);

    let n = 5;
    let f = recursive_fibonacci(n);
    assert_eq!(f, 5);

    let n = 10;
    let f = recursive_fibonacci(n);
    assert_eq!(f, 55);

    let n = 15;
    let f = recursive_fibonacci(n);
    assert_eq!(f, 610);

    let n = 19;
    let f = recursive_fibonacci(n);
    assert_eq!(f, 4181);
}
