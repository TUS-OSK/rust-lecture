fn main() {
    let x = 1;

    match x {
        0 => println!("0"),
        1 => println!("1"),
        2 | 3 => println!("2 | 3"),
        3..=10 => println!("3..=10"),
        _ => println!("all"),
    }
}