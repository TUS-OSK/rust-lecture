fn main() {
    let u1 = 1;
    let u2 = 2;

    let ans = func(&u1, &u2);
    
    println!("{}", ans);
}

fn func<'a>(u1: &'a u32, u2: &'a u32) -> &'a u32 {
    if *u1 > 0 {
        u1
    } else {
        u2
    }
}
