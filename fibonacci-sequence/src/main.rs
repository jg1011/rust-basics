use std::io; // Bring input output functionality into scope
use std::time::Instant; // For measuring runtime
use num_bigint::BigUint; // For dealing with large integers


fn main() {
    let mut n = String::new(); // Declare empty string as n

    println!("Input the Fibonnaci number you seek: ");

    io::stdin() // Classical line read logic, requires mutable reference as input
        .read_line(&mut n)
        .expect("failed to read line");

    let n: usize = n.trim().parse().expect("needs integer");

    // Recursive fibonnaci hits recursion depth pretty quickly, no good (max recursion depth is 128).

    // let rec_start = Instant::now(); // Get current time as instant object
    // let rec_n_fib = recursive_fibonacci(n);
    // let rec_duration = rec_start.elapsed().as_nanos(); // Use elapsed impl to get Duration object w/ elapsed time and return as u128 of nanos
    
    let iter_start = Instant::now(); // As before
    let iter_n_fib = iterative_fibonacci(n);
    let iter_duration = iter_start.elapsed().as_nanos(); // As before

    // println!("Fibonacci number {n} via recursion is {rec_n_fib}, which took {rec_duration} nanoseconds");
    println!("Fibonacci number {n} via iteration is \n {iter_n_fib} \n which took {iter_duration} nanoseconds to compute");
}   

// Bad function, only works for small n (should be n=64 and below ish, +- around 5)
fn recursive_fibonacci(n: usize) -> usize{
    // For edge cases, we need not handle 1 and two seperately as running with n=0 this way returns 0, easily seen 
    // to be fine. Then its just your classic recursion logic
    if n < 2 {
        return n;
    }
    // no need for an else, return breaks us out of function scope
    recursive_fibonacci(n-1) + recursive_fibonacci(n-2) // Expression so no semi-colon
}

fn iterative_fibonacci(n: usize) -> BigUint{
    // BigUint::new impl expects vector of digits, BigUint::ZERO does the 0 case for us
    let (mut a, mut b)= (BigUint::ZERO, BigUint::new(vec![1])); // 
    for _ in 1..=n-2{ // iterate up to n-2, first 2 are already complete
        // We point to b rather than use it directly, otherwise contents are moved and we get a compile time error
        let temp = a + &b; 
        a = b; 
        b = temp;
    }
    b
}
