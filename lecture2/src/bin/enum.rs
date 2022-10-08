fn main() {
    let st = St::new(0, 1.0, -1);

    let en_a = En::A(1);
    let en_b = En::B(1.0);
    let en_c = En::C(-1);
}

enum En {
    A(usize),
    B(f32),
    C(isize),
}

impl En {
    pub fn display(&self) {
        match self {
            En::A(value) => println!("a: {}", value),
            En::B(value) => println!("b: {}", value),
            En::C(value) => println!("c: {}", value),
            _ => panic!("Err")
        }
    }
}
 
struct St {
    pub a: usize,
    pub b: f32,
    pub c: isize,
}

impl St {
    pub fn new(a: usize, b: f32, c: isize) -> Self {
        Self {
            a,
            b,
            c
        }
    }
}

