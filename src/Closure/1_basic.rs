fn main(){
    // assign closure (inline-function) to variable is_even
    let is_even = |x| {
       x%2==0
    };
    let no = 13;
    // call closure by variable
    println!("{} is even ? {}",no,is_even(no));
}
