/* Jacob Collins
 * basic.rs
 * Overivew of basic datatypes, flow control, expressions, etc
 */

pub fn basic() {
    // ------------------------ VARS ------------------------
    // --- Constants ---
    const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // --- Shadowing ---
    // typically used for immutable vars
    let _x = 5; // <- immutable
    let _x = _x + 1;

    // allows us to change types within same variable
    let _spaces = "   "; // <- type: String()
    let _spaces = _spaces.len(); // <- type: usize

    // mutability would actually prevent this
    // let mut spaces = "   ";
    // spaces = spaces.len();   // <- COMPILER ERROR

    // -------------------- Data Types ----------------------
    // if many types are possible for the result, must add a
    // type annotation
    let _guess: u32 = "42".parse().expect("Not a number!");

    // ---- Scalar Types ---
    // represent a single value

    // Integer
    // type suffix
    let _num1 = 57u8; // 57 can be multiple types, u8 specifies
    let _num2 = 1000;
    let _num3 = 1_000;
    assert_eq!(_num2, _num3);

    // integer overflow in regular runtime causes panic!
    // integer overflow in --release gets twos compliment

    // Floating Point
    let _num4 = 2.0; // f64
    let _num5: f32 = 3.0; // f32

    // Character
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // --- Compound Types ---
    // --- Tuple ---
    // multiple types, fixed length
    let tup: (i32, f64, u8, char) = (500, 6.4, 1, 'a');
    // use pattern matching to destructure a tuple:
    let (_w, x, y, _z) = tup; // <- Destructuring
    println!("The value of y is: {y}");
    // or just reference index
    println!("(x) {x} = {} (tup.1)", tup.1);

    // --- Array ---
    // same type, fixed length
    let a = [1, 2, 3, 4, 5]; // <- useful if your data is const
                             // but probably rather use vec
    let _b: [i32; 5] = [1, 2, 3, 4, 5];

    // initialize with same value for each elem:
    let _c = [3; 5]; // [3, 3, 3, 3, 3];
    assert_eq!(a[0], 1); // Arrays are 0-indexed
                         // access with brackets

    // --------------- Statements/Expressions --------------
    // let y = let x = 3 <- Doesn't work
    let _y = {
        // New scope block allows
        let x = 3; // us to return something
        x + 1 // or just x   // when we normally couldn't
    };

    // --- if ---
    let condition: bool = true;
    if condition {
        println!("Do something");
    }
    // can use ifs in statements
    let _num6 = if condition { 5 } else { 7 };

    // ----------------------- Loops -----------------------
    // loop { ... } (retrying something that might fail)
    // Loop Labels
    let mut count = 0;
    'counting_up: loop {
        let mut remaining = 10;

        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // <- Specify which loop to break
            }
            remaining -= 1;
        }

        count += 1;
    }
    // --- while ---
    // while condition { ... }

    // --- for ---
    // best loop type, safe and concise
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("Value: {element}");
    }

    // .iter().enumerate() to generate index-elem pair
    for (index, elem) in a.iter().enumerate() {
        println!("a[{index}] = {elem}");
    }

    // range from..til (inclusive)..(notinclusive)
    // or rage from..=inclusive
    for number in (1..4).rev() {
        println!("{number}!");
    }

    // -------------------- Libraries ----------------------
    // --- Essential ---
    // Tokio - most popular async library
    //
    // Eyre - unifies error reports into a neater format
    // Color-Eyre - makes runtime exceptions as well formatted
    // as compiler exceptions
    //
    // Tracing - Errors trace back very well and give clearer
    // diagnostic information
}
