fn main() {
    let mut v = Vec::new();
    v.push(5);

    println!("V: {:?}", v);

    let y = vec![1, 2, 3, 4];
    println!("Y: {:?}", y);

    accessing();

    iteration();

    iteration_mutable();
}

fn from_enum() {
    enum SpreadsheetCell {
	Int(i32),
	Float(f64),
	Text(String),
    }

    let row = vec![
	SpreadsheetCell::Int(3),
	SpreadsheetCell::Text(String::from("blue")),
	SpreadsheetCell::Float(10.12),
    ];
}

fn accessing() {
    let v = vec![1, 2, 3, 4];
    //Access a reference
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    //Returns Option<&T>
    match v.get(2) {
	Some(third) => println!("The third {}", third),
	None => println!("There's no third"),
    }
}

fn iteration() {
    let v = vec![100, 23, 324];
    for i in &v {
	println!("{}", i);
    }
}

fn iteration_mutable() {
    let mut v = vec![23, 2932, 10];
    for i in &mut v {
	*i += 50;
    }

    println!("{:?}", v);
}
