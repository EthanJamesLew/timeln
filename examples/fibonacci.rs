//! A Naive Fibonacci Implementation to run timeln

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    println!("Printing Fibonacci Numbers 30-40...");
    for i in 30..=40 {
        println!("x[{}]={}", i, fibonacci(i));
    }
    println!("Finished.");
}
