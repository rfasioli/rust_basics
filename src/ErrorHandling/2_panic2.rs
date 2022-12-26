// This code not compile, by fatal error in code
fn main() {
    let a = [10,20,30];
    a[10]; //invokes a panic since index 10 cannot be reached
}
