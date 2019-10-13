extern crate rand;

use rand::Rng;

fn main() {
    // Given a list of integers, use a vector and return the mean (the average value),
    // median (when sorted, the value in the middle position), and mode
    // (the value that occurs most often; a hash map will be helpful here) of the list.
    let random_ints = generate_random_integers(10);
    println!("{:?}", random_ints);
}


fn generate_random_integers(quantity: u8) -> Vec<u8> {
    let mut random_ints: Vec<u8> = Vec::new();
    for x in 0..quantity {
	random_ints.push(rand::thread_rng().gen_range(0, 11));
    }
    
    random_ints
}
