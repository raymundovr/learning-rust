fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = String::from("abcde");
    let y = "xyz";

    println!("The longest is {}", longest(&x, &y));
}
