/* Jacob Collins
 * ownership.rs
 * Overivew ownership dynamics in rust
 */

pub fn ownership() {
    // Stack: first in, last out ( in/out <-> ...data )
    // - faster
    // - direct access to the data

    // Heap: allocater in random memory, returns pointers
    // - slower
    // - inderect access to the data

    // ------ Heap Data (e.g. strings) ------
    // anything that implements drop or needs pointers to manage data
    // is heap-allocated, and has to be cloned manually.

    // s1 invalid once s2 takes it
    let s1 = String::from("hello");
    let s2 = s1; // referred to as a move

    // println!("{}, world!", s1);   invalid because s1 no longer valid
    println!("{}, world!", s2);

    // clone() fixes this issue but uses more memory
    let s1 = String::from("hello");
    let s2 = s1.clone(); // referred to as a deep copy

    println!("{}, world!", s1);   
    println!("{}, world!", s2);

    // ------- Stack Data (e.g. integers) ------
    // integers
    // booleans
    // chars
    // tuples (containing only the above types)

    let x = 5;
    let y = x;
    println!("x: {x}, y: {y}"); // type size is known, so copy is done automatically

    // types with known sizes have the Copy() trait, so when they're used they're
    // copied automatically

    // types with Drop() CANNOT have Copy(), as it is implied that they need special
    // handling for their memory to be moved/copied/deleted

    // ------ Parameter Ownership ------
    let s1 = String::from("hello");
    // Ownership follows the ref, s1 is moved here and no longer accessible
    let _s2 = takes_and_gives_back(s1);

    fn takes_and_gives_back(str_ref: String) -> String {
        str_ref
    }
    
    // ------ References ------
    // either ONE mutable reference OR
    // any number of immutable references

    // ------ Slices ------
    // type of reference, so no ownership
    fn first_word(s: &String) -> String {

    }
}

