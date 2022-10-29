use std::thread;
use std::time::Duration;

fn main() {
    /*
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("vec: {:?}", v);
    });

    handle.join().unwrap();
    */

    let v = vec![1, 2, 3];
    
    let handle = thread::spawn(move || {
        println!("vec: {:?}", v);
    });

    handle.join().unwrap();
}