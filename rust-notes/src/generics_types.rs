/* Name: Jacob Collins
* Description:
Assessment of generics, types, and lifetimes
* Last Edited: August 18 2022
*/

pub fn generics_types() {
    mixup();
}

// Implementation of Point only for f32 type
// required because some math operations can only be done
// with f32
#[allow(dead_code)]
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// you can change the type generic label to make operations more clear
// You can also use different labels to notate operations where types
// are changing throughout the operations
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn mixup() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
