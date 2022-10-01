fn main() {
    let b = B {};
    let a = A { b }; //bにmoveが発生

    //AがBの所有者を保持している
}

struct A {
    b: B,
}

struct B {}