use std::ops::Deref;
use std::rc::Rc;

enum List {
    Cons(i32, Box<List>),
    Nil,
}
//Rc allows to hold multiple references to the same data
enum RCList {
    RCCons(i32, Rc<RCList>),
    RCNil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; //Defines an associated type for the Deref trait to use
    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping meeeeeee data {}", self.data);
    }
}

use crate::List::{Cons, Nil};
use crate::RCList::{RCCons, RCNil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my things..."),
    };
    let d = CustomSmartPointer {
        data: String::from("this as well..."),
    };

    println!("CustomSPs created.");

    let e = CustomSmartPointer {
        data: String::from("Drop me!"),
    };

    drop(e); //This is a call to std::mem::drop

    //First list
    let rca = Rc::new(RCCons(5, Rc::new(RCCons(10, Rc::new(RCNil)))));
    println!("count after creating a = {}", Rc::strong_count(&rca));
    //Now we create new lists holding references to rca.
    //Rc::clone() only increases the reference counter and does not deep clone
    let rcb = RCCons(3, Rc::clone(&rca));
    println!("count after creating b = {}", Rc::strong_count(&rca));
    {
        let rcc = RCCons(4, Rc::clone(&rca));
        println!("count after creating c = {}", Rc::strong_count(&rca));
    }
    println!("count after C out of scope = {}", Rc::strong_count(&rca));
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
