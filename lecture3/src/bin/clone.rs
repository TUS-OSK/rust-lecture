fn main() {
    let a = A {};
    let a_2 = a;
    //let a_3 = a;


    let b = B {};
    let b_2 = b.clone();
    let b_3 = b.clone();
}

struct A {}

#[derive(Clone)]
struct B {}