// no parameters and no return value
// main function is the entry point of the program
mod variables_mutability;
mod onwership;
mod struct_sample;
mod enum_sample;
mod common_collections;


fn main(){
    create_spreadsheet();
    println!("the value is {}",another_function(5)); //argument
}

fn another_function(x: i32) -> i32{ //parameter
    let y = {
        let x = 3;
        x + 1 //expression
    };

    use hello_cargo::hosting;
    //Create a function to avoid syntax error
    if false {
        rand::thread_rng();
        hosting::add_to_waitlist();
        variables_mutability::call_all_functions();
        onwership::call_all_functions();
        struct_sample::call_all_functions();
        enum_sample::call_all_functions();
        common_collections::call_all_functions();
    }
    x + y
}

#[allow(dead_code)]
#[derive(Debug)]
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

    let mut row1: Vec::<SpredsheetCell> = Vec::new();

    for cell in row {
        println!("{:?}", cell);
        row1.push(cell);
    }
}