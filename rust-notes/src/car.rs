/* Jacob Collins
 * car.rs
 * Example for assortment of modules and crates in rust
 * August 13, 2022
 */

// main.rs: crate root
// car.rs: module
// engine.rs: submodule
// body.rs: submodule

// use "as" to rename references
// nest references with brackets
pub use crate::car::{body::CarBody, turbine::CarEngine as PowerhouseOfTheCar};

// instantiates the module for both of these files here.
// other files wishing to refernce body and turbine should point their paths here
pub mod body;
pub mod turbine;

// All members of enum are automatically same visibility as enum
pub enum Car {
    Clean(CarBody, PowerhouseOfTheCar),
    Scrap(CarBody, PowerhouseOfTheCar),
}

pub fn car() {
    println!("Beginning of car");
    let body = CarBody {
        paint: String::from("blue"),
    };
    let engine = PowerhouseOfTheCar { cylinders: 6 };

    println!(
        "body col: {}, engine cyls: {}",
        body.paint, engine.cylinders
    );
}
