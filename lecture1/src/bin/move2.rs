fn main() {
    let s = String::from("hello");

    take_ownership(s); //sがmove
}  

fn take_ownership(s: String) {
    println!("{}", s);
}