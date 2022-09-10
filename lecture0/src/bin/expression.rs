fn main() {
    let block = {};

    let value = 0;

    let i = if value == 0 {
        true
    } else {
        false
    };

    let f = for i in 0..1 {};

    let m = match value {
        0 => true,
        _ => false,
    };

    println!("i: {:?}, f: {:?}, m: {:?}", i, f, m);
}