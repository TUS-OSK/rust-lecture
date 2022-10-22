fn main() {
    let a = A {};
    let a_2 = a;
    //let a_3 = a;


    let b = B {};
    let b_2 = b;
    let b_3 = b;
}

struct A {}

#[derive(Clone, Copy)]
struct B {}