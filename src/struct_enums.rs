// This program demonstrates the use of structs and enums in Rust
use std::cell::Cell;

struct User {
    pub username: String, // immutable field
    pub email: String,    // immutable field
    pub sign_in_count: Cell<u64>, // mutable field
    pub active: Cell<bool>,       // mutable field
    
}

struct Color(i32, i32, i32);

// unit-like struct
// when you want to create a struct that doesn't have any fields, you can use a unit-like struct
// This type of struct is useful in situations in which you need to implement a trait on a type 
// but don't have any data that you want to store in the type itself.
struct Location;

fn create_location() -> Location {
    Location
}

fn create_user() -> User {
    let user1 = User {
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: Cell::new(323),
        active: Cell::new(true),
    };

    // Modify the mutable fields
    user1.sign_in_count.set(user1.sign_in_count.get() + 1);
    user1.active.set(false);

    let user2 = User {
        username: String::from("user2"),
        email: String::from("anotheruser@example.com"),
        sign_in_count: Cell::new(user1.sign_in_count.get()), // clone the value
        active: Cell::new(user1.active.get()),               // clone the value
    };

    println!("Username: {}", user1.username);
    println!("user 2 Email: {}", user2.email);
    println!("Sign-in count: {}", user1.sign_in_count.get());
    println!("Active: {}", user1.active.get());

    user1
}

fn create_color_location() -> Color {
    let black = Color(0, 0, 0);
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("Black: {}, {}, {}", black.0, black.1, black.2);
    println!("Tup: {}, {}, {}", tup.0, tup.1, tup.2);
    black
}

pub fn call_all_functions(){
    create_user();
    create_color_location();
    create_location();
}