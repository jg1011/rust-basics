use std::io; // Bring input output functionality into scope

fn main() {
    let mut n = String::new(); // Declare empty string as n

    println!("Input the Fibonnaci number you seek: ");

    io::stdin() // Classical line read logic, requires mutable reference as input
        .read_line(&mut n)
        .expect("failed to read line");

    let n: u32 = n.trim().parse().expect("needs integer");

    let rec_n_fib = recursive_fibonacci(n);
    let iter_n_fib = iterative_fibonacci(n);

    println!("Fibonacci number {n} via recursion is {rec_n_fib}");
    println!("Fibonacci number {n} via iteration is {iter_n_fib}");
}   

fn recursive_fibonacci(n: u32) -> u32{
    // For edge cases, we need not handle 1 and two seperately as running with n=0 this way returns 0, easily seen 
    // to be fine. Then its just your classic recursion logic
    if n < 2 {
        return n;
    }
    // no need for an else, return breaks us out of function scope
    recursive_fibonacci(n-1) + recursive_fibonacci(n-2) // Expression so no semi-colon
}

fn iterative_fibonacci(n: u32) -> u32{
    let (mut a, mut b)= (1,1); // 
    for _ in 1..=n-2{ // iterate up to n-2, first 2 are already complete
        let temp = a + b;
        a = b; 
        b = temp;
    }
    b
}
