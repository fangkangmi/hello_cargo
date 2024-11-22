#![allow(dead_code)]
fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
        crate::serve_order();
    }

    fn cook_order() {}

    pub enum Appetizer {
        Soup,
        Salad,
    }

}

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub use crate::front_of_house::hosting;
// use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    hosting::add_to_waitlist();
}


use rand::Rng;
use std::io::{self, Write}; //use std::io and std::io::Write
use std::collections::*; //use all types in std::collections

// Dummy function to use rand::Rng
fn use_rand() {
    let mut rng = rand::thread_rng();
    let _random_number: u32 = rng.gen();
}

// Dummy function to use std::io::{self, Write}
fn use_io() -> io::Result<()> {
    let mut buffer = io::stdout();
    buffer.write_all(b"Hello, world!")?;
    buffer.flush()?;
    Ok(())
}

// Dummy function to use std::collections
fn use_collections() {
    let mut map = HashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");

let mut set = HashSet::new();
    set.insert("a");
    set.insert("b");
}