use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

// Drop work like a destructor
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("dropping MyBox object from memory ");
    }
}

fn main() {
    let x = 50;
    let a = MyBox::new(x);
    println!("{}", *a);
    let b = MyBox::new("Hello");
    println!("{}", *b);
}
