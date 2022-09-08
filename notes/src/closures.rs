/*
Name: Jacob Collins
Description:
Notes on using closures
Last Edited: September 07 2022
*/

pub fn closures() {
    // Closures have a lot of optional syntax
    //     fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    //     let add_one_v2 = |x: u32| -> u32 { x + 1 };
    //     let add_one_v3 = |x|             { x + 1 };
    //     let add_one_v4 = |x|               x + 1  ;

    // It's like a mini function!
    let maximum = |x| x + 1;
    let num = 20;
    let plus_one = maximum(num);
    println!("{plus_one}");

    let mut set = vec![1, 2, 3, 4, 5];
    println!("Before push_10 closure: {:?}", set);
    let mut push_10 = || set.push(10);
    push_10();
    println!("After push_10 closure: {:?}", set);
}
