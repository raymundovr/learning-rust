fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;

    println!("Values {} {} {}", x, y, z);
    another(5);
    statements_expressions();
    println!("Five {} + 1 = {}", five(), plus_one(5));
    conditional();
    cycle();
    let mut s = String::from("Hello world");
    let word = first_word_string(&s);
    println!("S {} word at {}", s, word);

    s.clear();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_string(s: &String) -> &str {
    &s[0..first_word(&s)]
}

fn another(x: i32) {
    println!("Received {}", x);
}


fn statements_expressions() {
    let y = {
        let x = 5;
        x + 1 //No semicolon!!!
    };

    println!("Y {}", y);
}

fn five() -> i32 {
    5
} //Again no semicolon

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn conditional() {
    let number = if true {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}


fn cycle() {
    let mut counter = 5;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The loop result is {} for {}", result, counter);

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("While {}", a[index]);

        index += 1;
    }

    for element in a.iter() {
        println!("For {}", element);
    }

    for number in (1..4).rev() {
        println!("De reversa {}", number);
    }
}
