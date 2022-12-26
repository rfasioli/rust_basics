fn main() {
    with_string();
    with_array();
}

fn with_string() {
    let n1 = "Tutorials".to_string();
    println!("length of string is {}",n1.len());
    let c1 = &n1[4..9];

    // fetches characters at 4,5,6,7, and 8 indexes
    println!("{}",c1);
}

fn with_array() {
    let data = [10,20,30,40,50];
    use_slice(&data[1..4]);
    //this is effectively borrowing elements for a while
}

fn use_slice(slice:&[i32]) {
    // is taking a slice or borrowing a part of an array of i32s
    println!("length of slice is {:?}",slice.len());
    println!("{:?}",slice);
}
