use std::fs::File;
fn main() {
   let f = File::open("main.jpg");
   //this file does not exist
   println!("{:?}",f);
}

// ## The output is: `Err(Error { repr: Os { code: 2, message: "No such file or directory" } })`
