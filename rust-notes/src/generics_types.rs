/* Name: Jacob Collins
* Description:
Assessment of generics, types, and lifetimes
* Last Edited: August 18 2022
*/

pub fn generics_types() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
