use std::fs::File;
fn main(){
   let f = File::open("pqr.txt").expect("File not able to open");
   //The expect function define a custom message `File not able to open` in case of a panic (file does not exist)
   println!("end of main");
}
