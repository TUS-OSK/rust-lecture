fn main() {

}

/* 
fn animal_bark(animal: &impl Animal) {
    animal.bark();
}
*/

fn animal_bark<T: Animal>(animal: &T) {
    animal.bark();
}

fn bark_and_ride<T: Animal + Vehicle>(target: &T) {
    target.bark();
    target.ride();
}

trait Vehicle {
    fn ride(&self);
}

trait Animal {
    fn bark(&self);
}

struct Cat {}

impl Cat {
    fn new() -> Self {
        Self {}
    }
}

impl Animal for Cat {
    fn bark(&self) {
        println!("Nyaa");
    }
}

struct Dog {}

impl Animal for Dog {
    fn bark(&self) {
        println!("Bowwow");
    }
}

