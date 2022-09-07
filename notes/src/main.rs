/* Jacob Collins
 * main.rs
 * Combines notes etc for output and testing
 */

use notes::closures::closures;

fn main() {
    // Main can be allowed to return Result values, so you can use ? on functions in main()

    // Rust and C both return 0 from their executables if the function processed successfully
    // A non-zero return value represents some kind of error

    closures();
}
