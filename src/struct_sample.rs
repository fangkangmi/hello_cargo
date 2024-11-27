// This program demonstrates the use of structs and enums in Rust
use std::cell::Cell;

struct User {
    pub username: String, // immutable field
    pub email: String,    // immutable field
    pub sign_in_count: Cell<u64>, // mutable field
    pub active: Cell<bool>,       // mutable field
    
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

struct Color(i32, i32, i32);

fn create_color_location() -> Color {
    let black = Color(0, 0, 0);
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("Black: {}, {}, {}", black.0, black.1, black.2);
    println!("Tup: {}, {}, {}", tup.0, tup.1, tup.2);
    black
}

// unit-like struct
// when you want to create a struct that doesn't have any fields, you can use a unit-like struct
// This type of struct is useful in situations in which you need to implement a trait on a type 
// but don't have any data that you want to store in the type itself.
struct Location;

fn create_location() -> Location {
    Location
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    // use & to borrow the struct instead of taking ownership
    fn area(&self) -> u32{
        self.width * self.height
    }

    // associated function
    // functions that don't take self as a parameter are called associated functions
    fn square(size: u32) -> Rectangle{
        Rectangle {width: size, height: size}
    }
}

// methods are different from functions in that they're defined within the context of a struct
impl Rectangle {
    // method with multiple parameters
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}


fn area_struct(){
    let rect =Rectangle {width: 30, height: 50};
    println!("The area of the rectangle is {} square pixels.",rect.area());
    println!("rect1 is {:#?}", rect);

    let rect2 = Rectangle {width: 10, height: 40};
    let rect3 = Rectangle {width: 60, height: 45};
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));
    
    let s = Rectangle::square(3);
    println!("s is {:#?}", s);
}



pub fn call_all_functions(){
    create_user();
    create_color_location();
    create_location();
    area_struct();
}