use std::collections::HashMap;

fn main(){
    let mut state_codes = HashMap::new();
    state_codes.insert("KL","Kerala");
    state_codes.insert("MH","Maharashtra");
    println!("size of map is {}",state_codes.len());
    println!("{:?}",state_codes);

    match state_codes.get(&"KL") {
       Some(value)=> {
          println!("Value for key KL is {}",value);
       }
       None => {
          println!("nothing found");
       }
    }

    // Iterate
    for (key, val) in state_codes.iter() {
        println!("key: {} val: {}", key, val);
    }

    // contains
    if state_codes.contains_key(&"GJ") {
        println!("found key");
    }

    // Insert and remove
    state_codes.insert("GJ","Gujarat");
    println!("length of the hashmap {}",state_codes.len());
    state_codes.remove(&"GJ");
    println!("length of the hashmap after remove() {}",state_codes.len());
}
