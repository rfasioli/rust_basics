fn main() {
    // Simple array
    let arr:[i32;4] = [10,20,30,40];
    println!("array is {:?}",arr);
    println!("array size is :{}",arr.len());

    // Loop with index
    for index in 0..arr.len() {
        println!("index is: {} & value is : {}",index,arr[index]);
    }

    // Iterate array
    for val in arr.iter(){
        println!("value is :{}",val);
    }

    // without a type
    let arr = [10,20,30,40];
    println!("array is {:?}",arr);
    println!("array size is :{}",arr.len());

    // Default values
    let arr:[i32;4] = [-1;4];
    println!("array is {:?}",arr);
    println!("array size is :{}",arr.len());

    // Mutable array
    let mut arr:[i32;4] = [10,20,30,40];
    arr[1] = 0;
    println!("{:?}",arr);

    // Pass array by value
    let arr = [10,20,30];
    update(arr);
    println!("Inside main {:?}",arr);

    // Pass array by reference
    let mut arr = [10,20,30];
    update_by_ref(&mut arr);
    println!("Inside main {:?}",arr);

    const N: usize = 20;
    // pointer sized
    let arr = [0; N];
    println!("{}",arr[10])

}

fn update(mut arr:[i32;3]){
    for i in 0..3 {
        arr[i] = 0;
    }
    println!("Inside update {:?}",arr);
}

fn update_by_ref(arr:&mut [i32;3]){
    for i in 0..3 {
        arr[i] = 0;
    }
    println!("Inside update {:?}",arr);
}
