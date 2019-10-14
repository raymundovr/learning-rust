//Convert strings to pig latin. The first consonant of each word is moved to the end of
//the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel
//have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the
//details about UTF-8 encoding!

use std::io;

fn main() {
    println!("Input your string");
    let mut entry = String::new();

    io::stdin()
        .read_line(&mut entry)
        .expect("Failed to read line");

    println!(
        "You entered: {}\nPig latin equivalent: {}",
        entry,
        convert_to_pig_latin(&entry)
    );
}

fn convert_to_pig_latin(entry: &String) -> String {
    let words: Vec<&str> = entry.split(' ').collect();
    let mut converted = String::from("");

    for w in &words {
        converted = format!("{} {}", converted, &convert_word(w));
    }
    converted
}

fn convert_word(word: &str) -> String {
    let converted = String::from(word);
    let first_character = word.chars().next().unwrap();

    first_character.to_string()
}
