//! A Script That Sleeps and Prints Patterns
use std::thread;
use std::time::Duration;


fn main() {
    println!("Printing patterns...");
    for i in 0..=10 {
        thread::sleep(Duration::from_secs(1));
        println!("Iteration {}", i);
    }
    println!("Finished.");
}