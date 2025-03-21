use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use num_bigint::BigUint;
use num_traits::{Zero, One};
use clap::Parser;

/// Command-line arguments configuration
#[derive(Parser)]
struct Cli {
    /// Total execution time in seconds
    #[arg(short, long, default_value_t = 10)]
    duration: u64,
}

/// Fibonacci number calculator using dynamic programming
fn fibonacci(n: usize) -> BigUint {
    // Handle base cases
    if n == 0 {
        return Zero::zero();
    } else if n == 1 {
        return One::one();
    }

    // Sleep to simulating a slow operation.
    if n == 10005 {
        thread::sleep(Duration::from_millis(50));
    }

    // Initialize DP table with explicit type annotation
    let mut dp: Vec<BigUint> = vec![Zero::zero(); n + 1];
    dp[1] = One::one();

    // Iterative calculation to avoid recursion
    for i in 2..=n {
        dp[i] = &dp[i - 1] + &dp[i - 2];
    }

    dp[n].clone()
}

fn main() {
    // Parse command-line arguments
    let cli = Cli::parse();
    let total_duration = Duration::from_secs(cli.duration);

    // Atomic flag for controlling thread execution
    let running = Arc::new(AtomicBool::new(true));

    // Shared results storage with thread-safe access
    let results = Arc::new(Mutex::new(vec![Zero::zero(); 10]));
    let start_index = 10000;

    // Create and manage worker threads
    let mut handles = vec![];
    for (idx, n) in (start_index..start_index + 10).enumerate() {
        let running = Arc::clone(&running);
        let results = Arc::clone(&results);

        handles.push(thread::spawn(move || {
            // Continuous execution loop
            while running.load(Ordering::Relaxed) {
                let value = fibonacci(n);

                // Update shared results
                let mut res = results.lock().unwrap();
                res[idx] = value.clone();

                // Prevent excessive CPU usage
                thread::sleep(Duration::from_millis(1));
            }
        }));
    }

    // Main thread timing control
    let start_time = Instant::now();
    while start_time.elapsed() < total_duration {
        // Periodic results printing
        let res = results.lock().unwrap();
        println!("Current results: {:?}", *res);
        drop(res); // Explicit lock release

        // Console update interval
        thread::sleep(Duration::from_secs(1));
    }

    // Signal threads to stop
    running.store(false, Ordering::Relaxed);

    // Wait for thread termination
    for handle in handles {
        handle.join().unwrap();
    }

    // Final output
    let final_res = results.lock().unwrap();
    println!("Final results: {:?}", *final_res);
    println!("Program completed in {:?} seconds", total_duration);
}