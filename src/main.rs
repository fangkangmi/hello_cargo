// no parameters and no return value
// main function is the entry point of the program

const MAX_POINTS: u32 = 100_000;
fn main(){
    variable_and_constant();

    shadowing_example();
}


fn variable_and_constant() {
    // println! is a macro that prints text to the console
    // ! indicates that it is a macro
    // The difference between a macro and a function is that a macro is a code generator that generates code at compile time
    // A function is a piece of code that is executed at runtime
    println!("Hello, World!");

    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    //Constants are always immutable, and their type must be annotated
    // Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of the program need to know about.
    // Constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.
    const MAX_POINTS: u32 = 100_000;
}


fn shadowing_example() {
    //shadowing: declare a new variable with the same name as a previous variable, and the new variable shadows the previous variable
    let x = 5;
    println!("The value of x is: {}", x);

    let x = x + 1;
    println!("The value of x is: {}", x);

    let x = x * 2;
    println!("The value of x is: {}", x);

    // Shadowing allows us to change the type of the variable
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);
}
//Rust is ahead of its time in terms of safety and performance.
//Ahead of its time is a phrase that means that something is more advanced than its contemporaries.