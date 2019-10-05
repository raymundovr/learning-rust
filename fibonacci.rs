use std::io;

fn main() {    
    println!("Which order? ");
    let mut number = String::new();

    io::stdin().read_line(&mut number)
        .expect("Failed to read line");

    println!("Calculating Fibonacci for {} order", number);

    let number: i32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => println!("Error"),
    };

    let fib = fibonacci(number);
}

fn fibonacci( n: i32 ) -> i32 {
    if (n == 0) || (n == 1) {
        1
    }
    0
}