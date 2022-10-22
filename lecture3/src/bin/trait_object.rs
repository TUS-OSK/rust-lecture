fn main() {
}

trait Draw {
    fn draw(&self);
}

struct Button {}

struct TextField {}

impl Draw for Button {
    fn draw(&self) {
        println!("button draw");
    }
}

impl Draw for TextField {
    fn draw(&self) {
        println!("text field draw");
    }
}


struct Screen {
    pub component: Vec<Box<dyn Draw>>,
}

