use std::rc::Rc;

fn main() {
    let a = Rc::new(1);
 
    println!("strong_count: {}", Rc::strong_count(&a));
    {
        let b = a.clone();

        println!("strong_count: {}", Rc::strong_count(&a));
        {
            let c = b.clone();
            println!("strong_count: {}", Rc::strong_count(&a));
        } 
        println!("strong_count: {}", Rc::strong_count(&a));
    }
    println!("strong_count: {}", Rc::strong_count(&a));
}