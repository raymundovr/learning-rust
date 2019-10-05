#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//Methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //Associated
    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

fn main() {
    let rect1 = Rectangle {width: 30, height: 50};
    let rect2 = Rectangle {width: 10, height: 40};
    let rect3 = Rectangle {width: 60, height: 50};
    //println!("The area for {:#?} is {}", rect1, area(&rect1));
    println!("The area for {:#?} is {}", rect1, rect1.area());
    println!("Can rect1 hold? rect2: {}, rect3: {}",
             rect1.can_hold(&rect2),
             rect1.can_hold(&rect3));
    let sq = Rectangle::square(3);
    println!("Square {:#?}", sq);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
