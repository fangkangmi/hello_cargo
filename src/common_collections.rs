#![allow(unused_mut)]
#![allow(dead_code)]

fn demonstrate_vectors() {
    let _empty_vector: Vec<i32> = Vec::new();
    // _empty_vector.push(5); //error[E0596]: cannot borrow immutable local variable `_empty_vector` as mutable

    let _initialized_vector = vec![1, 2, 3];
    let mut modifiable_vector = Vec::new();
    modifiable_vector.push(5);
    modifiable_vector.pop();

    // Reading elements of vectors
    let mut vector = vec![1, 2, 3, 4, 5];
    // & is a reference to the value, which does not have ownership
    // The reference allows you to share the value without transferring ownership
    // if we use vector[0], we would have to transfer ownership.
    // this would mean that we could not use vector again
    let first_element = &vector[0];

    // vector.push(6);  //error[E0502]: cannot borrow `vector` as mutable because it is also borrowed as immutable
    println!("The first element is: {}", first_element);

    // Use get method to return an Option<&T>
    match vector.get(2) {
        Some(number) => println!("The third element is {}", number),
        None => println!("There is no third element."),
    }
}

#[allow(unused_mut)]
pub fn demonstrate_borrowing() {
    let mut vector = vec![100, 32, 57];
    let first_element = &vector[0];

    //vector.push(6); //error[E0502]: cannot borrow `vector` as mutable because it is also borrowed as immutable
    
    println!("The first element is: {}", first_element);
    vector.push(6); //error[E0502]: cannot borrow `vector` as mutable because it is also borrowed as immutable

    // if we want to use the first element after pushing a new element
    // we need to create a new reference
    let _first_element = &vector[0];

    // Iterating over immutable references
    for i in &vector {
        println!("{}", i);
    }

    // Iterating over immutable references with index
    for (index, value) in vector.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }

    // Iterating over mutable references
    for i in &mut vector {
        *i += 50;
    }
}

fn demonstrate_strings() {
    // Rust string is a growable, mutable, owned, UTF-8 encoded string type
    // UTF-8 is a variable-width encoding that can represent any Unicode code point
    let mut _empty_string = String::new();
    let data = "initial contents";
    let _string_from_data = data.to_string();
    let _string_from_literal = "initial content".to_string();
    let _string_from_function = String::from("initial content");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // let _s3 = s1.clone() + &s2; // s1 is still valid here
    let _s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    // println!("s1: {}", s1); //error[E0382]: borrow of moved value: `s1`

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // format! macro doesn't take ownership of any of its parameters
    let formatted_string = format!("{}-{}-{}", s1, s2, s3);
    println!("Formatted string: {}", formatted_string);
}

// String is a wrapper over a Vec<u8>
fn demonstrate_string_operations() {
    let hello = "Здравствуйте";
    // Unicode scalar values range from U+0000 to U+D7FF and U+E000 to U+10FFFF
    // 3: 208,151
    println!("Length of 'Здравствуйте': {}", hello.len()); // 24

    // chars() method returns a char type
    for c in hello.chars() {
        println!("{}", c);
    }

    // bytes of the string is: [208, 151, 208, 176, 209, 128, 209, 129, 208, 178, 208, 190, 208, 179, 209, 143]
    for b in hello.bytes() {
        println!("{}", b);
    }

    let answer = "नमस्ते";
    // ['न', 'म', 'स', '्', 'त', 'े']
    // but the last character is actually two characters
    // We want ["न", "म", "स्", "ते"]
    // We can use the grapheme_clusters method from the unicode-segmentation crate
    for c in answer.chars() {
        println!("{}", c);
    }
}

fn demonstrate_string_slicing() {
    let s = String::from("Здравствуйте");
    // let h = &s[0]; //error[E0277]: the type `str` cannot be indexed by `{integer}`

    let _h = &s[0..3];
    // println!("{}", h); // panic! because the index is not at a character boundary
    // (a1, b1) (a2, ||||  b2)  

    let h = &s[0..4];
    println!("{}", h);
}

fn demonstrate_hash_map_basic_operations() {
    use std::collections::HashMap;

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Score of team {}: {:?}", team_name, score);

    match score {
        Some(s) => println!("Score of team {}", s),
        None => println!("team does not exist")
    }

    // Iterating over a HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Overwriting a value
    scores.insert(String::from("Blue"), 25);
    
    // or_insert method only inserts if the key does not already have a value
    let e = scores.entry(String::from("Red")); //e = Entry(VacantEntry("Red"))
    e.or_insert(50);
    
    scores.entry(String::from("Yellow")).or_insert(50); // Yellow already has a value, so it is not updated

    println!("{:?}", scores);
}

#[allow(unused)]
fn demonstrate_hash_map_from_vectors() {
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // zip method creates a vector of tuples
    // collect method is used to convert the vector of tuples into a HashMap
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Blue");
    let field_value = String::from("Yellow");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_value are invalid at this point
}

fn demonstrate_word_count() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";
    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert returns a mutable reference to the value
        let count: &mut i32 = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn main() {
    demonstrate_vectors();
}

pub fn call_all_functions() {
    main();
    demonstrate_borrowing();
    demonstrate_strings();
    demonstrate_string_operations();
    demonstrate_string_slicing();
}