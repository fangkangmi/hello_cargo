#![allow(unused_mut)]
#![allow(dead_code)]
fn vectors(){
    let _v1: Vec<i32> = Vec::new();
    //v1.push(5); //error[E0596]: cannot borrow immutable local variable `_v1` as mutable

    let _v2 = vec![1, 2, 3];
    let mut v3 = Vec::new();
    v3.push(5);
    v3.pop();

    //reading elements of vectors
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    // v.push(6);  //error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    println!("The first element is: {}", first);

    //use get method to return an Option<&T>
    match v.get(2){
        Some(number) => println!("The third element is {}", number),
        None => println!("There is no third element."),
    }
}

#[allow(unused_mut)]
fn borrowing(){
    let mut v = vec![100, 32, 57];
    let first = &v[0];
    // v.push(6); //error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    println!("The first element is: {}", first);

    //Iterating over mutable references
    for i in &v{
        println!("{}", i);
    }

    //Iterating over mutable references
    for (index, value) in v.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }

    // * is used to dereference the value
    // here we use * to change the value of i
    for i in &mut v{
        *i += 50;
    }
}

enum SpredsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}

fn create_spreadsheet(){
    let row = vec![
        SpredsheetCell::Int(3),
        SpredsheetCell::Text(String::from("blue")),
        SpredsheetCell::Float(10.12),
    ];

    let _v1: Vec<i32> = Vec::new();
    let mut row1: Vec::<SpredsheetCell> = Vec::new();

    
    for cell in row {
        row1.push(cell);
    }
}

fn string(){
    // Rust string is a growable, mutable, owned, UTF-8 encoded string type
    // UTF-8 is a variable-width encoding that can represent any Unicode code point
    let mut _s = String::new();
    let data = "initial contents";
    let _s = data.to_string();
    let _s  = "inintial content".to_string();
    let _s = String::from("initial content");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // let _s3 = si.clone() + &s2; // s1 is still valid here
    let _s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    // println!("s1: {}", s1); //error[E0382]: borrow of moved value: `s1`

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // format! macro doesn't take ownership of any of its parameters
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {}", s);
}

// String is a wrapper over a Vec<u8>
fn string2(){
    let hello = "Здравствуйте";
    // Unicode scalar values range from U+0000 to U+D7FF and U+E000 to U+10FFFF
    // 3: 208,151
    println!("{}", hello.len()); //24

    // chars() method returns a char type
    for c in hello.chars(){
        println!("{}", c);
    }

    // bytes of the string is: [208, 151, 208, 176, 209, 128, 209, 129, 208, 178, 208, 190, 208, 179, 209, 143]
    for b in hello.bytes(){
        println!("{}", b);
    }

    let answer = "नमस्ते";
    // ['न', 'म', 'स', '्', 'त', 'े']
    // but the last character is actually two characters
    // We want ["न", "म", "स्", "ते"]
    // We can use the grapheme_clusters method from the unicode-segmentation crate
    for c in answer.chars(){
        println!("{}", c);
    }
}

fn string3(){
    let s = String::from("Здравствуйте");
    // let h = &s[0]; //error[E0277]: the type `str` cannot be indexed by `{integer}`

    let h = &s[0..3];
    // println!("{}", h); // panic! because the index is not at a character boundary
    // (a1, b1) (a2, ||||  b2)  

    let h = &s[0..4];
    println!("{}", h);
}

fn main(){
    vectors();
}

pub fn call_all_functions(){
    main();
    borrowing();
}