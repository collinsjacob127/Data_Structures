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

    // Iteration
    print!("v: [");
    for i in &v {
        print!("{}, ", i);
    }
    println!("]\n");

    // works with mutable
    let mut v = vec![1, 2, 3, 4, 5];

    print!("v * 10: [");
    for i in &mut v {
        *i *= 10;
        print!("{}, ", *i);
    }
    println!("]\n");

    // using enums with vecs to store multiple types in a vec
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row: Vec<SpreadSheetCell> = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(6.9),
        SpreadSheetCell::Text(String::from("Example String")),
    ];

    // ----- STRING -----
    // collection of characters
    // a vector with special changes
    let mut _s = String::new();

    let data = "Initial contents";

    let _s = data.to_string();

    // UTF-8 encoded so can be pretty flexible
    println!("hello");
    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);

    let goodbye = String::from(", y adios");

    // Takes the contents of hello for itself
    // just borrows goodbye
    // don't usually use + for strings unless quick

    //     let pleasantries = hello + &goodbye;
    //     println!("{}", pleasantries);

    // format!() does NOT take ownership of any params
    let pleasantries = format!("{}{}", hello, goodbye);
    println!("{}", pleasantries);

    // note:
    // string[x] does not work because characters can sometimes
    // be multiple bytes and returns would get weird

    // string[x..y] does work though because it can return the
    // full values of the characters it references
    // Throws up an error if accessing only a partial character
    // at one of the ends, besides that is all good.

    // ----- HASHMAPS -----
    // Map of paired values
}
