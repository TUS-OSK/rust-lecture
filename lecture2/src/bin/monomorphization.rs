fn main() {

}

trait Animal {
    fn bark(&self);
}

struct Cat {}

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

/*
fn animal_bark<T: Animal>(animal: T) {
    animal.bark();
}
*/
//このようなメソッドはコンパイル時に...


fn animal_bark_cat(animal: Cat) {
    animal.bark();
}

fn animal_bark_dog(animal: Dog) {
    animal.bark();
}

//となる