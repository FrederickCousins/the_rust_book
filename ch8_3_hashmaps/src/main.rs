// Hash Maps are not included by the prelude so we need to introduce them
use std::collections::HashMap;

fn main() {
    // Iniitialising a hash map does not require you to introduce its type
    let mut scores = HashMap::new();

    // but once initialised all keys and values must by of the same type respectively
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name)
        .copied() // means we get an Option<i32> rather than Option<&i32>
        .unwrap_or(0); // so that if the key does not exist we get a defailt value

    // similar to Python, we can iterate over the keys and values of a hash map
    for (key, value) in &scores {
        println!("key: {key}, \t value: {value}");
    }

    // Ownership
    // for items on the stack that implement Copy, values are copied in 
    // for items on the heap that don't, values are moved and ownership given to the hash map
    // e.g.

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // println!("{field_name}");

    // How to do the common, "if field does not exist, insert"
    scores.entry(String::from("Blue")).or_insert(100);
    // this is much neater than writing out the logic each time

    // How to update a value based on the old value

    let text = "hello world oh wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word)
            .or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    // lovely!

    

}   
