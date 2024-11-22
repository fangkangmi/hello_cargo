#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Implement a method on an enum
impl IpAddrKind {
    fn call(&self) {
        println!("Calling IpAddrKind method");
    }
}

fn option_sample() {
    let some_number = Some(5);

    let absent_number: Option<i32> = None;

    let x = 5;
    let y = x + some_number.unwrap_or(0) + absent_number.unwrap_or(0);

    println!("y: {}", y);
}

fn find_user_by_id(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

fn option_sample2() {
    let user = find_user_by_id(1);
    match user {
        Some(name) => println!("Found user: {}", name),
        None => println!("User not found"),
    }
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin:Coin) ->u8{
    match coin{
        Coin::Penny =>{
            println!("Lucky penny!");
            1
        },
        Coin::Nickel =>5,
        Coin::Dime =>10,
        Coin::Quarter(state) =>{
            println!("State quarter from {:?}", state);
            25
        },
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match must be exhaustive (all possible values must be covered) 
    // or you can use _ to match all other values
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}



fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    // Using match to take ownership
    match home {
        IpAddrKind::V4(a, b, c, d) => {
            println!("Home IP: {}.{}.{}.{}", a, b, c, d);
        }
        _ => (),
    }

    loopback.call();

    // loopback can still be used because it was not moved
    // if let is a more concise way to write match
    if let IpAddrKind::V6(addr) = loopback {
        println!("Loopback IP: {}", addr);
    }

    // home cannot be used here because it was moved
    // println!("Hello, home {:?}

    let penny = Coin::Penny;
    println!("Value in cents: {}", value_in_cents(penny));

    let quarter = Coin::Quarter(UsState::Alaska);
    println!("Value in cents: {}", value_in_cents(quarter));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six: {:?}", six);
    println!("none: {:?}", none);

}

pub fn call_all_functions() {
    main();
    option_sample();
    option_sample2();
}