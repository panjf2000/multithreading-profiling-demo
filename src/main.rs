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

    /// Number of worker threads
    #[arg(short = 't', long, default_value_t = 10)]
    threads: usize,

    /// Sleep duration in milliseconds for even numbers
    #[arg(short = 's', long = "sleep-ms", default_value_t = 50)]
    sleep_ms: u64,

    /// Starting index for Fibonacci calculations
    #[arg(short = 'i', long = "start-index", default_value_t = 10000)]
    start_index: usize,
}

/// Fibonacci number calculator with even-number sleep
fn fibonacci(n: usize, sleep_ms: Duration) -> BigUint {
    // Handle base cases
    if n == 0 {
        return Zero::zero();
    } else if n == 1 {
        return One::one();
    }

    // Sleep for even numbers
    if n % 2 == 0 {
        thread::sleep(sleep_ms);
    }

    // Initialize DP table
    let mut dp: Vec<BigUint> = vec![Zero::zero(); n + 1];
    dp[1] = One::one();

    // Iterative calculation
    for i in 2..=n {
        dp[i] = &dp[i - 1] + &dp[i - 2];
    }

    dp[n].clone()
}

fn main() {
    // Parse command-line arguments
    let cli = Cli::parse();
    let total_duration = Duration::from_secs(cli.duration);
    let sleep_ms = Duration::from_millis(cli.sleep_ms);

    // Thread control flag
    let running = Arc::new(AtomicBool::new(true));

    // Shared results storage
    let results = Arc::new(Mutex::new(vec![Zero::zero(); cli.threads]));

    // Create worker threads
    let mut handles = vec![];
    for (idx, n) in (cli.start_index..cli.start_index + cli.threads).enumerate() {
        let running = Arc::clone(&running);
        let results = Arc::clone(&results);
        let sleep_ms = sleep_ms;

        handles.push(thread::spawn(move || {
            // Continuous calculation loop
            while running.load(Ordering::Relaxed) {
                let value = fibonacci(n, sleep_ms);

                // Update shared results
                let mut res = results.lock().unwrap();
                res[idx] = value.clone();

                // Prevent excessive CPU
                thread::sleep(Duration::from_millis(1));
            }
        }));
    }

    // Main control loop
    let start_time = Instant::now();
    while start_time.elapsed() < total_duration {
        // Print periodic results
        let res = results.lock().unwrap();
        println!(
            "Fibonacci[{}-{}]: {:?}",
            cli.start_index,
            cli.start_index + cli.threads - 1,
            *res
        );
        drop(res);

        thread::sleep(Duration::from_secs(1));
    }

    // Stop threads
    running.store(false, Ordering::Relaxed);

    // Cleanup
    for handle in handles {
        handle.join().unwrap();
    }

    // Final output
    let final_res = results.lock().unwrap();
    println!(
        "Final results for {}-{}: {:?}",
        cli.start_index,
        cli.start_index + cli.threads - 1,
        *final_res
    );
    println!("Program ran for {:?} seconds", cli.duration);
}