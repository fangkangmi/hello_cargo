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

fn main(){
    vectors();
}

pub fn call_all_functions(){
    main();
    borrowing();
}