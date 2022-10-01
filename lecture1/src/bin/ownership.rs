fn main() {
    let s1 = String::from("Hello");

    println!("s1: {}", s1);

    let s2 = s1;

    //moveが発生してるのでs1は使用することができない
    //println!("s1: {}", s1);
    println!("s2: {}", s2);

    {
        let s1 = String::from("Hello");    
        let s2 = s1;
    }
}image.png