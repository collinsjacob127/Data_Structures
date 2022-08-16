/* Jacob Collins
 * car.rs
 * Example for assortment of modules and crates in rust
 * August 13, 2022
 */

// main.rs: crate root
// car.rs: module
// engine.rs: submodule
// body.rs: submodule

use crate::car::body::CarBody;
// use "as" to rename references
use crate::car::turbine::CarEngine as PowerhouseOfTheCar;

pub mod body;
pub mod turbine;

// All members of enum are automatically same visibility as enum
pub enum Car {
    Clean(CarBody, PowerhouseOfTheCar),
    Scrap(CarBody, PowerhouseOfTheCar),
}

pub fn car() {
    let body = CarBody {
        paint: String::from("blue"),
    };
    let engine = PowerhouseOfTheCar { cylinders: 6 };

    println!(
        "body col: {}, engine cyls: {}",
        body.paint, engine.cylinders
    );
}
