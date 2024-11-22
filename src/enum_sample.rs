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
    if let IpAddrKind::V6(addr) = loopback {
        println!("Loopback IP: {}", addr);
    }

    // home cannot be used here because it was moved
    // println!("Hello, home {:?}
}

pub fn call_all_functions() {
    main();
}