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
    let maximum = |x| x + 1;
    let num = 20;
    let plus_one = maximum(num);
    println!("{plus_one}");
}
