// Jacob Collins
// First.rs
// Basic linked list implementation
// July 13, 2022

// pub = public 
// That means List can be used outside of this module
pub enum List {
    Empty,
    Elem(i32, Box<List>),
}
