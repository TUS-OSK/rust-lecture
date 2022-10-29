use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("aaa");
        tx.send(val).unwrap();
    });

    let recv = rx.recv().unwrap();

    println!("{}", recv);
}