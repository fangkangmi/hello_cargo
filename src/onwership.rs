fn move_ownership() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // error[E0382]: use of moved value: `s1`
    println!("{}", s2);
}

fn clone_and_modify() {
    let mut s1 = String::from("hello");
    s1.push_str("world");

    let s2 = s1.clone();

    let mut s3 = "text".to_string();
    s3.push_str(". 2");

    println!("s1 = {}, s2 = {}, s3 = {}", s1, s2, s3);
}

fn copy_integer() {
    let x = 5;
    println!("x is {}", x);
    let y = x.clone();
    println!("x is {}, y is {}", x, y);
}

fn take_and_copy() {
    let s = String::from("hello");
    take_ownership(s);

    let x = 5;
    makes_copy(x);

    println!("x: {}", x);
}

fn transfer_ownership() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1: {}, s3: {}", s1, s3);
}

fn reference_and_borrowing() {
    let s1: String = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
}

// dangling reference example
// error[E0106]: missing lifetime specifier
// fn dangle() -> &String {
    // let s = String::from("hello");
    // &s
// }

fn slice() {
    let s = String::from("hello world");
    let word_index = first_world(&s);

    println!("The first word is {}", &s[..word_index]);
}

pub fn call_all_functions() {
    move_ownership();
    clone_and_modify();
    copy_integer();
    take_and_copy();
    transfer_ownership();
    reference_and_borrowing();
    slice();
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(x: i32) {
    println!("{}", x);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // b' ' is byte literal
            return i;
        }
    }
    s.len()
}

