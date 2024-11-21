// no parameters and no return value
// main function is the entry point of the program
mod variables_mutability;
mod onwership;
mod struct_enums;

fn main(){
    println!("the value is {}",another_function(5)); //argument
}

fn another_function(x: i32) -> i32{ //parameter
    let y = {
        let x = 3;
        x + 1 //expression
    };

    //Create a function to avoid syntax error
    if false {
        variables_mutability::call_all_functions();
        onwership::call_all_functions();
        struct_enums::call_all_functions();
    }
    x + y
}