fn main() {
    let var_i32 = 5; //stack
    let b = Box::new(var_i32);  //box store data into heap
    println!("b = {}", b);

    let x = 5; //value type variable
    let y = Box::new(x); //y points to a new value 5 in the heap

    println!("{}",5==x);
    println!("{}",5==*y); //dereferencing y
}
