use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    if let Some(value) = map.get(&1) {
        println!("value for key 1 is: {}", value);
    } else {
        println!("Key 1 not found");
    }

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{}", secret_number);
}
