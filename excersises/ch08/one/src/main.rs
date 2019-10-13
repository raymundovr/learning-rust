extern crate rand;

use rand::Rng;

fn main() {
    // Given a list of integers, use a vector and return the mean (the average value),
    // median (when sorted, the value in the middle position), and mode
    // (the value that occurs most often; a hash map will be helpful here) of the list.
    let random_ints = generate_random_integers(10);
    println!("Set of integers {:?}", random_ints);

    println!("Mean: {}", calculate_mean(&random_ints));
}


fn generate_random_integers(quantity: u8) -> Vec<u8> {
    let mut random_ints: Vec<u8> = Vec::new();
    let mut x = 0;
    while x < quantity {
	random_ints.push(rand::thread_rng().gen_range(0, 11));
	x += 1;
    }
    
    random_ints.sort()
}


fn calculate_mean(integers: &Vec<u8>) -> f32 {
    let mut sum = 0.0;

    for i in integers {
	sum += f32::from(*i);
    }

    println!("--Total sum: {}", sum);
    
    sum / (integers.len() as f32)
}

//fn calculate_median(integers: &Vec<u8>) -> u8 {
//    
//}
