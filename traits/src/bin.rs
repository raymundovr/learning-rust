extern crate traitslib;

use traitslib::{NewsArticle, Summary, Tweet, Unrecognized};

pub fn notify(item: impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn main() {
    let tweet = Tweet {
        username: String::from("raybachas"),
        content: String::from("blah ..."),
        reply: false,
        retweet: false,
    };

    let unknown = Unrecognized {
        id: String::from("ID-1"),
        source: String::from("http://www.source.com/"),
    };

    println!("1 new tweeet: {}", tweet.summarize());
    println!("Others: {}", unknown.summarize());
}
