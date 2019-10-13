extern crate rand;

use rand::Rng;
use std::collections::HashMap;

fn main() {
    // Given a list of integers, use a vector and return the mean (the average value),
    // median (when sorted, the value in the middle position), and mode
    // (the value that occurs most often; a hash map will be helpful here) of the list.
    let random_ints = generate_random_integers(10);
    println!("Set of integers {:?}", random_ints);

    println!("Mean: {}", calculate_mean(&random_ints));
    println!("Median: {}", calculate_median(&random_ints));
    println!("Mode: {:?}", calculate_mode(&random_ints));
}

fn generate_random_integers(quantity: u8) -> Vec<u8> {
    let mut random_ints: Vec<u8> = Vec::new();
    let mut x = 0;
    while x < quantity {
        random_ints.push(rand::thread_rng().gen_range(1, 11));
        x += 1;
    }

    random_ints.sort();
    random_ints
}

fn calculate_mean(integers: &Vec<u8>) -> f32 {
    let mut sum = 0.0;

    for i in integers {
        sum += f32::from(*i);
    }

    println!("--Total sum: {}", sum);

    sum / (integers.len() as f32)
}

fn calculate_median(integers: &Vec<u8>) -> u8 {
    let middle = integers.len() / 2;
    match integers.get(middle) {
        Some(median) => *median,
        None => 0,
    }
}

fn calculate_mode(integers: &Vec<u8>) -> HashMap<u8, u8> {
    let mut mode: HashMap<u8, u8> = HashMap::new();
    for i in integers {
        let count = mode.entry(*i).or_insert(0);
        *count += 1;
    }
    mode
}
