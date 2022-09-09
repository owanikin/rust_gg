// struct CustomSmartPointer {
//     data: String,
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data `{}`!", self.data);
//     }
// }

// fn main() {
//     // let c = CustomSmartPointer { data: String::from("my stuff") };
//     // let d = CustomSmartPointer { data: String::from("other stuff") };
//     // println!("CustomSmartPointers created.");
//     let c = CustomSmartPointer { data: String::from("Some data") };
//     println!("CustomSmartPointer created.");
//     drop(c);
//     println!("CustomSmartPointer dropped before the end of main.");
// }


enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5,
        Rc::new(Cons(10,
            Rc::new(Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a));
    
    let b = Cons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));
    let c = Cons(6, Rc::clone(&a));
    println!("Count after creating c = {}", Rc::strong_count(&a));
    {
        let d = Cons(4, Rc::clone(&a));
        println!("Count after creating d = {}", Rc::strong_count(&a));
    }
    
    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
}
