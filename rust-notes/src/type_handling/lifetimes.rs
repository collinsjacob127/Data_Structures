/* Name: Jacob Collins
* Description:
Notes on lifetimes, their uses, implementations, etc.
* Last Edited: August XX 2022
*/

pub fn lifetimes() {
    //     &i32        // a reference
    //     &'a i32     // a reference with an explicit lifetime
    //     &'a mut i32 // a mutable reference with an explicit lifetime

    /*
    This function would not compile without lifetimes being explicitly given

    This lets the compiler know that the scope of these references can be
    handed off wherever the function is called, rather than handled here.

    'a here states that the returned str   \/  will only last so long as both
    of the references passed to the function remain in scope
    */
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string is long"); // Scope of x

    {
        let string2 = String::from("xyz"); // Scope of y
        let result = longest(string1.as_str(), string2.as_str()); // Return param requires both x
                                                                  // and y
        println!("The longest string is {}", result); // This print is allowed
    }
    // println!("The longest string is {}", result); // This print is not
}
