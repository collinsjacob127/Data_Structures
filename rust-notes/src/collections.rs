/* Jacob Collins
 * collections.rs
 * Discusses Vectors, Strings, and Hashmaps
 * August 16, 2022
 */

pub fn collections() {
    println!("Beginning of collections");
    // Collections:
    // Vectos, Strings, Hashmaps -> HEAP-ALLOCATED

    // ----- VECTOR -----
    // variable number of values next to eachother
    // vecs use generics so its good to specify type or generic

    // Initializations
    let _v = vec![1, 2, 3]; // Macro to initialize with values
    let mut v: Vec<i32> = Vec::new(); // Initialize empty and typed
    println!("{}", v.len());

    // Adding new vals
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    v.pop();

    println!("{}", v.len());

    // Referencing vals
    // direct (doesn't cover errors):
    let _second: &i32 = &v[1];

    // .get() (covers errors):
    if let Some(second) = v.get(1) {
        println!("The second element is {}", second);
    }
    // remember that you can't have an immutable and mutable ref at once

    // Iteration
    print!("v: [");
    for i in &v {
        print!("{}, ", i);
    }
    println!("]\n");

    // works with mutable
    let mut v = vec![1, 2, 3, 4, 5];
    
    print!("v: [");
    for i in &mut v {
        print!("{}, ", *i);
    }
    println!("]\n");

    // ----- STRING -----
// collection of characters
    
    // ----- HASHMAPS -----
    // Map of paired values
}
