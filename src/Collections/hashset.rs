use std::collections::HashSet;
fn main() {
    let mut names = HashSet::new();

    names.insert("Mohtashim");
    names.insert("Kannan");
    names.insert("TutorialsPoint");
    names.insert("Mohtashim");//duplicates not added

    println!("size of the set is {}",names.len());
    println!("{:?}",names);

    //Iterate
    for name in names.iter() {
        println!("{}",name);
    }

    // get
    match names.get(&"Mohtashim"){
        Some(value)=>{
           println!("found {}",value);
        }
        None =>{
           println!("not found");
        }
    }

     // contains
    if names.contains(&"Kannan") {
        println!("found name");
    }

    //Remove
    println!("length of the Hashset: {}",names.len());
    names.remove(&"Kannan");
    println!("length of the Hashset after remove() : {}",names.len());
}
