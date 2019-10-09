#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrComposed {
    V4(u8, u8, u8, u8),
    V6(String),
}

//Another enum definition
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        //method body
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
	Coin::Penny => 1,
	Coin::Nickel => 5,
	Coin::Dime => 10,
	Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
	None => None,
	Some(i) => Some(i + 1),
    }
}

fn main() {
    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home_composed = IpAddrComposed::V4(127, 0, 0, 1);

    println!("Home {:?}", home);
    println!("Loppback {:?}", loopback);
    println!("Home Composed {:?}", home_composed);

    let m = Message::Write(String::from("Hello"));
    m.call();

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Done {:?} {:?} {:?}", five, six, none);
}
