use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;
use std::collections::HashMap;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn fibonacci_range(start: u64, end: u64) {
    let now = Instant::now();
    for i in start..end {
        println!("Fibonacci({}) = {}", i, fibonacci(i));
    }
    let elapsed = now.elapsed();
    println!("Thread finished in: {:?}", elapsed);
}

fn sequential_mode(array: &Vec<i32>) -> (i32, usize, Duration) {
    let now = Instant::now();
    let mut counts: HashMap<i32, usize> = HashMap::new();

    for &num in array {
        *counts.entry(num).or_insert(0) += 1;
    }

    let mut most_frequent = 0;
    let mut max_count = 0;

    for (&num, &count) in &counts {
        if count > max_count {
            most_frequent = num;
            max_count = count;
        }
    }
    let elapsed = now.elapsed();
    (most_frequent, max_count, elapsed)
}

fn parallel_mode(array: &Vec<i32>, num_threads: usize) -> (i32, usize, Duration) {
    let now = Instant::now();
    let chunk_size = array.len() / num_threads;
    let mut handles = vec![];

    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = if i == num_threads - 1 {
            array.len()
        } else {
            (i + 1) * chunk_size
        };

        let chunk = array[start..end].to_vec();

        let handle = thread::spawn(move || {
            let mut counts: HashMap<i32, usize> = HashMap::new();

            for &num in &chunk {
                *counts.entry(num).or_insert(0) += 1;
            }

            let mut most_frequent = 0;
            let mut max_count = 0;

            for (&num, &count) in &counts {
                if count > max_count {
                    most_frequent = num;
                    max_count = count;
                }
            }

            (most_frequent, max_count)
        });

        handles.push(handle);
    }

    let mut global_counts: HashMap<i32, usize> = HashMap::new();

    for handle in handles {
        let (most_frequent, max_count) = handle.join().unwrap();
        *global_counts.entry(most_frequent).or_insert(0) += max_count;
    }

    let mut most_frequent = 0;
    let mut max_count = 0;

    for (&num, &count) in &global_counts {
        if count > max_count {
            most_frequent = num;
            max_count = count;
        }
    }

    let elapsed = now.elapsed();
    (most_frequent, max_count, elapsed)
}

fn main() {
    println!("Fibonacci Calculation with Threads:");

    let thread1 = thread::spawn(|| fibonacci_range(0, 10));
    let thread2 = thread::spawn(|| fibonacci_range(10, 20));

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("\nArray Processing:");

    let array_size = 100_000;
    let mut rng = rand::rng();
    let array: Vec<i32> = (0..array_size).map(|_| rng.random_range(-200..100)).collect();

    println!("Sequential Mode:");
    let (most_frequent_seq, max_count_seq, time_seq) = sequential_mode(&array);
    println!("Most frequent element: {}, count: {}, time: {:?}", most_frequent_seq, max_count_seq, time_seq);

    println!("\nParallel Mode:");
    let num_threads_list = vec![1, 2, 4, 8, 16];
    for num_threads in &num_threads_list {
        let (most_frequent_par, max_count_par, time_par) = parallel_mode(&array, *num_threads);
        println!("Threads: {}, Most frequent element: {}, count: {}, time: {:?}", num_threads, most_frequent_par, max_count_par, time_par);
    }
}