use std::thread;
use std::time::Duration;

fn main() {
    //Thread returns JoinHandle type
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    //What happens when we call it in before the main thread?
    //handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    //By saving it to handle we guarantee that the thread is run to completion
    //The join() forces the main thread to wait
    //Placing the join correctly makes the thread to run at the same time
    handle.join().unwrap();

    vector();
}

fn vector() {
    let v = vec![1, 2, 3];
    //Using move the closure takes ownership of the values
    let handle = thread::spawn(move || {
        println!("Here's a vector {:?}", v);
    });

    handle.join().unwrap();
}
