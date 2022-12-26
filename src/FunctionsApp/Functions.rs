
fn main(){
    fn_hello(); //calling a function
    println!("pi value is {}",get_pi()); // returning a value
    test_passing_values();

    let name:String = String::from("TutorialsPoint");
    display(name); // cannot access name after display
}


 //Defining a function
fn fn_hello(){
    println!("hello from function fn_hello ");
}

// Returning a value
fn get_pi()->f64 {
    22.0/7.0
}

fn test_passing_values() {
    let no:i32 = 5;
    mutate_no_to_zero(no); //Passing by value
    println!("The value of no is:{}",no);

    let mut no:i32 = 5;
    mutate_no_to_zero_by_ref(&mut no); //Passing by reference
    println!("The value of no is:{}",no);
}

fn mutate_no_to_zero(mut param_no: i32) {
    param_no = param_no*0;
    println!("param_no value is :{}",param_no);
}

fn mutate_no_to_zero_by_ref(param_no:&mut i32){
    *param_no = 0; //de reference
}

fn display(param_name:String){
    println!("param_name value is :{}",param_name);
}
