use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    // Generic structure with static method new
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0 //returns data (borrowing)
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    // calling static method

    println!("5==x is {}", 5 == x);
    println!("5==*y is {}", 5 == *y);
    // dereferencing y
    println!("x==*y is {}", x == *y);
    //dereferencing y
}
