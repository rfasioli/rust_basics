fn main() {
    // new
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);

    println!("size of vector is :{}",v.len());
    println!("{:?}",v);

    // Initializing
    let mut v = vec![1,2,3];
    println!("{:?}",v);
    println!("size of vector is :{}",v.len());

    v.push(10);

    if v.contains(&10) {
        println!("found 10");
    }
    println!("size of vector is :{}",v.len());

    // Iterating
    for i in &v {
        println!("{}",i);
    }
    println!("{:?}",v);

}
