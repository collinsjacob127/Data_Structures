/* Jacob Collins
 * structs.rs
 * Outline of how to use structs for structuring data
 * August XX, 2022
 */

pub fn structs() {
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

    fn make_rect(dims: (u32, u32)) -> Rect {
        Rect {
            _width: dims.0,
            _height: dims.1,
        }
    }

    fn area(rect: &Rect) -> u32 {
        rect._width * rect._height
    }

    let rect1 = make_rect((20, 50));
    println!("Area of rect1: {}", area(&rect1));
    dbg!(&rect1);
}
