/* Jacob Collins
 * enums.rs
 * Analysis of enum and pattern matching functionality
 * August 11, 2022
 */

pub fn enums() {
    // Enums are basically much more flexible structs
    //
    // Identifiers
    enum IpAddrKind {
        V4,
        V6,
    }

    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    // Different types
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("This"));

    // Implement methods on enums
    enum Message {
        _Quit,
        _Move { x: i32, y: i32 },
        Write(String),
        _ChangeColor(i32, i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // Do something
            unimplemented!();
        }
    }

    let m = Message::Write(String::from("Test Message says hello!"));
    m.call();

    // Match syntax
    enum Coin {
        Penny,
        _Nickel,
        _Dime,
        _Quarter,
    }

    impl Coin {
        fn value_in_cents(&self) -> u8 {
            match self {
                Coin::Penny => 1,
                Coin::_Nickel => 5,
                Coin::_Dime => 10,
                Coin::_Quarter => 25,
            }
        }
    }

    let cent = Coin::Penny;
    println!("A cent is {} cents!", cent.value_in_cents());

    // Catch-all patterns
    // other:
    // handle else case using value
    let dice_roll = 7;
    match dice_roll {
        3 => println!("Three! Huzzah!"),
        7 => println!("Seven! Huzzah!"),
        other => println!("{}! Huzzah!", other),
    }

    // _ character:
    // handle else case but don't use value
    let dice_roll = 7;
    match dice_roll {
        3 => println!("Three! Huzzah!"),
        7 => println!("Seven! Huzzah!"),
        _ => (),
    }

    // if - let pattern matching
    // first badly using match:
    let name = Some("Johnny");
    match name {
        Some(identifier) => println!("{}", identifier),
        _ => (),
    }

    // using if let:
    if let Some(identifier) = name {
        println!("{}!", identifier);
    }
}
