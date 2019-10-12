use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //println!("{:?}", scores);

    for (key, value) in &scores {
	println!("{}: {}", key, value);
    }

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match &score {
	Some(val) => println!("Score: {}", val),
	None => println!("Not found!")
    }

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let inital_scores = vec![10, 50];

    //Zip: create a vector of tuples
    //Collect turns it into a hashmap because we specify HashMap<_, _>
    let scores2: HashMap<_, _> = teams.iter().zip(inital_scores.iter()).collect();
    println!("{:?}", scores2);

    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 10);
    scores3.insert(String::from("Blue"), 25);
    //If Yellow is not there insert 50
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    
    println!("{:?}", scores3);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
	let count = map.entry(word).or_insert(0);
	*count += 1;
    }

    println!("{:?}", map);
}
