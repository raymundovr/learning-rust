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
        entry.trim(),
        convert_to_pig_latin(&entry)
    );
}

fn convert_to_pig_latin(entry: &String) -> String {
    let words: Vec<&str> = entry.split(' ').collect();
    let mut converted = String::from("");
    let vowels = ["a", "e", "i", "o", "u"];

    for w in &words {
        //let first_char = w.chars().next().unwrap();
        let (first_char, last_chars) = w.split_at(1);
        let mut converted_word;
        if vowels.contains(&first_char) {
            converted_word = format!("{}-hay", w);
        } else {
            converted_word = format!("{}-{}ay", last_chars, first_char);
        }
        converted = format!("{} {}", converted, converted_word);
    }
    converted
}
