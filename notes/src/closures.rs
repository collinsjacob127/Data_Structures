/*
Name: Jacob Collins
Description:
Notes on using closures
Last Edited: September 07 2022
*/
use std::thread;

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
    //     println!("After push_10 closure: {:?}", set); <- this doesn't work because mutable borrow
    //                                                      from closure definition followed by immutable
    //                                                      borrow from print statement

    push_10();
    println!("After push_10 closure: {:?}", set);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

/*
*We’ll first examine how we can use closures to capture values from the environment they’re defined in for later use. Here’s the scenario: Every so often, our t-shirt company gives away an exclusive, limited-edition shirt to someone on our mailing list as a promotion. People on the mailing list can optionally add their favorite color to their profile. If the person chosen for a free shirt has their favorite color set, they get that color shirt. If the person hasn’t specified a favorite color, they get whatever color the company currently has the most of.

There are many ways to implement this. For this example, we’re going to use an enum called MeatType that has the variants Mean and Lean (limiting the number of colors available for simplicity). We represent the company’s inventory with an Inventory struct that has a field named shirts that contains a Vec<MeatType> representing the shirt colors currently in stock. The method giveaway defined on Inventory gets the optional shirt color preference of the free shirt winner, and returns the shirt color the person will get. This setup is shown in Listing 13-1:

*/

#[allow(dead_code)]
enum MeatType {
    Lean,
    Mean,
}

#[allow(dead_code)]
struct Inventory {
    shirts: Vec<MeatType>,
}

#[allow(dead_code)]
impl Inventory {
    fn giveaway(&self, user_preference: Option<MeatType>) -> MeatType {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> MeatType {
        let mut mean = 0;
        let mut lean = 0;

        for color in &self.shirts {
            match color {
                MeatType::Mean => mean += 1,
                MeatType::Lean => lean += 1,
            }
        }
        if mean > lean {
            MeatType::Mean
        } else {
            MeatType::Lean
        }
    }
}
