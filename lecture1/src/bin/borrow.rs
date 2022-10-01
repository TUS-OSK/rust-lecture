fn main() {
    let mut s1 = String::from("hello");

    borrow(&s1);
    borrow_mut(&mut s1);
}

fn borrow(s: &String) {
    println!("{}", s);
}

fn borrow_mut(s: &mut String) {
    s.push_str(", world!");

    println!("{}", s);
}