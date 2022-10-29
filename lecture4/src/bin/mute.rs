use std::{thread, vec};
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn main() {
    /*
    let v = Mutex::new(vec![1, 2, 3]);
    
    let handle = thread::spawn(|| {
        println!("vec: {:?}", &v);
    });

    let handle2 = thread::spawn(|| {
        *v.lock().unwrap() = vec![4];
        println!("vec: {:?}", &v);
    });

    handle.join().unwrap();
    handle2.join().unwrap();
    */

    let v = Arc::new(Mutex::new(vec![1, 2, 3]));
    let v1 = v.clone();
    let v2 = v.clone();
    
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));

        println!("1 vec: {:?}", v1);
    });

    let handle2 = thread::spawn(move || {
        *v.lock().unwrap() = vec![4];
        println!("2 vec: {:?}", v2);
    });

    handle.join().unwrap();
    handle2.join().unwrap();

}