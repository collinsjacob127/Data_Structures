#[ignore = "dead_code"]
fn main() {
    // ------------------------ VARS ------------------------
    // --- Constants ---
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // --- Shadowing ---
    // typically used for immutable vars
    let x = 5; // <- immutable
    let x = x + 1;

    // allows us to change types within same variable
    let spaces = "   ";         // <- type: String()
    let spaces = spaces.len();  // <- type: usize

    // mutability would actually prevent this
    // let mut spaces = "   ";
    // spaces = spaces.len();   // <- COMPILER ERROR

    // -------------------- Data Types ----------------------
    // if many types are possible for the result, must add a 
    // type annotation
    let guess: u32 = "42".parse().expect("Not a number!");
    
    // ---- Scalar Types ---
    // represent a single value

    // Integer
    // type suffix
    let num1 = 57u8; // 57 can be multiple types, u8 specifies
    let num2 = 1000;
    let num3 = 1_000;
    assert_eq!(num2, num3);
    
    // integer overflow in regular runtime causes panic!
    // integer overflow in --release gets twos compliment

    // Floating Point
    let num4 = 2.0;         // f64
    let num5: f32 = 3.0;    // f32

    // Character 
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // --- Compound Types ---

}
