// no parameters and no return value
// main function is the entry point of the program
mod variables_mutability;
mod onwership;

fn main(){
    println!("the value is {}",another_function(5)); //argument
    onwership::call_all_functions();
}

fn another_function(x: i32) -> i32{ //parameter
    let y = {
        let x = 3;
        x + 1 //expression
    };

    //Create a function to avoid syntax error
    if false {
        variables_mutability::all_data_types();
        onwership::call_all_functions();
    } 
    x + y
}