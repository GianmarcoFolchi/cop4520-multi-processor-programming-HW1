use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let upper_bound = 100_000_000; 
    let prime_counter = Arc::new(Mutex::new(1));
    let primes = Arc::new(Mutex::new(Vec::new()));
    let start = Instant::now();
    let handles: Vec<_> = (0..8).map(|_| {
        let prime_counter = Arc::clone(&prime_counter);
        let primes = Arc::clone(&primes);

        thread::spawn(move || {
            loop {
                let prime_to_check = {
                    let mut counter = prime_counter.lock().unwrap();
                    if *counter >= upper_bound {
                        break;
                    }
                    let current = *counter;
                    *counter += 1;
                    current
                };

                if is_prime(prime_to_check) {
                    let mut primes_lock = primes.lock().unwrap();
                    primes_lock.push(prime_to_check);
                }
            }
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
    let duration_in_secs = start.elapsed().as_secs_f64();
    let primes = primes.lock().unwrap();
    let sum: i64 = primes.iter().map(|&x| x as i64).sum();
    let last_ten_primes = &primes[primes.len() - 10..];
    let mut file = File::create("primes.txt")?;
    let metrics = format!("{}, {}, {}, {:?}", duration_in_secs, primes.len(), sum, last_ten_primes);
    file.write_all(metrics.as_bytes())?;
    file.sync_all()?;
    Ok(())
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    return true
}