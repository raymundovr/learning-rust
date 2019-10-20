use std::fs::File;
use std::io;
use std::io::{Read,ErrorKind};

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
	Ok(file) => file,
	Err(error) => match error.kind() {
	    ErrorKind::NotFound => match File::create("hello.txt") {
		Ok(fc) => fc,
		Err(e) => panic!("Problem creating the file {:?}", e),
	    },
	    other_error => panic!("Problem opening the file {:?}", other_error),
	},
    };

    println!("{:?}", read_username_from_file(f));
    println!("{:?}", read_username_from_file_shorter());
}

fn read_username_from_file(f: File) -> Result<String, io::Error> {
    let mut s = String::new();

    let mut f = f;
    //Original sample
    // match f.read_to_string(&mut s) {
    // 	Ok(_) => Ok(s),
    // 	Err(e) => Err(e),
    // }

    // Using ? operator. Returns errors to the caller using it.
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
