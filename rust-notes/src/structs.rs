/* Jacob Collins
 * structs.rs
 * Outline of how to use structs for structuring data
 * August XX, 2022
 */
use core::cmp::{max, min};

fn structs() {
    println!("Beginning of structs");
    // Define struct
    struct User {
        _active: bool,
        username: String,
        email: String,
        _sign_in_count: u64,
    }

    // Field Init Shorthand: Same name? no need to set equal manually
    fn _build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            _active: true,
            _sign_in_count: 1,
        }
    }
    // --- Instantiate Struct ---
    let mut user1 = User {
        email: String::from("weschan@gmail.com"),
        username: String::from("Plimbismoe"),
        _active: true,
        _sign_in_count: 1,
    };

    // --- Access and Mutate struct elems ---
    user1.username = String::from("BATMAN");
    user1.email = String::from("evenbetter@gmail.com");

    // --- Struct Update Syntax ---
    // Struct update syntax: copy all values from another
    // struct except for the ones you want
    let _user2 = User {
        email: String::from("poggest@gmail.com"),
        ..user1
    };

    // user1 is invalid after the above assignment
    // because the string has been moved into user2
    // from user1 so the pointer is gone for user1

    // ----- Tuple Structs -----
    // no named fields
    // simpler alternative to structs for lightweight scenarios

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // Example Struct Program

    // #[derive(Debug)] lets us print the contents of a Rect
    #[derive(Debug)]
    struct Rect {
        _width: u32,
        _height: u32,
    }

    let rect1 = Rect::new(20, 50);
    println!("Area of rect1: {}", rect1.area());
    dbg!(&rect1);

    // ----- Methods ------
    // struct functions

    impl Rect {
        fn new(_width: u32, _height: u32) -> Self {
            Rect { _width, _height }
        }

        fn area(&self) -> u32 {
            self._width * self._height
        }

        fn _width(&self) -> u32 {
            self._width
        }

        fn max(&self) -> u32 {
            max(self._width, self._height)
        }

        fn min(&self) -> u32 {
            min(self._width, self._height)
        }

        fn can_hold(&self, rect2: &Rect) -> bool {
            self.max() > rect2.max() && self.min() > rect2.min()
        }
    }

    let rect1 = Rect {
        _width: 30,
        _height: 50,
    };
    let rect2 = Rect {
        _width: 10,
        _height: 40,
    };
    let rect3 = Rect {
        _width: 60,
        _height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
