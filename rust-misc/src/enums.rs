/* Jacob Collins
 * enums.rs
 * Analysis of enum and pattern matching functionality
 * August 11, 2022
 */

pub fn enums() {
    // Enums are basically much more flexible structs
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("This"));
}
