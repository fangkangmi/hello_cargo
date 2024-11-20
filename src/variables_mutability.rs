

pub fn variable_and_constant() {
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
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}


pub fn shadowing_example() {
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

pub fn data_type(){
    //Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.
    //Scalar types represent a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
    //Integers are a number without a fractional component. Rust has signed and unsigned integers. Signed integers can be positive, negative, or zero. Unsigned integers can only be positive or zero.
    //Signed integers are i8, i16, i32, i64, i128, and isize. The number after i indicates the number of bits the integer type takes up.
    //Unsigned integers are u8, u16, u32, u64, u128, and usize. The number after u indicates the number of bits the integer type takes up.
    //Integer literals can be expressed in decimal, hexadecimal, octal, binary, and byte (u8 only) formats.
    //Floating-point numbers are numbers with a fractional component. Rust has two primitive types for floating-point numbers: f32 and f64. The default type is f64 because on modern CPUs it's roughly the same speed as f32 but is capable of more precision.
    //Boolean types are either true or false. Booleans are one byte in size.
    //Character types are specified with single quotes, as opposed to strings, which use double quotes. Rust's char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent more than just ASCII.
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of t is: {}", t);
    println!("The value of f is: {}", f);
    println!("The value of c is: {}", c);
    println!("The value of z is: {}", z);
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    //isize and usize depend on the kind of computer your program is running on: 64 bits if you're on a 64-bit architecture and 32 bits if you're on a 32-bit architecture.
    //The primary situation in which you'd use isize or usize is when indexing some sort of collection.
    let x: isize = 10;
    let y: usize = 10;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

pub fn compound_types(){
    //tuple: a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    //Tuples are useful for functions that need to return more than one value.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of tup is: {}, {}, {}", x, y, z);
    println!("The value of tup is: {}, {}, {}", tup.0, tup.1, tup.2);

    //array: every element of an array must have the same type. Arrays in Rust have a fixed length, like tuples.
    //Arrays are useful when you want your data allocated on the stack rather than the heap.

    //The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out.
    //All data stored on the stack must have a known, fixed size.

    //the heap is less organized: when you put data on the heap, you request a certain amount of space. 
    //The operating system finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location in memory.

    let month: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let first = month[0];
    let second = month[1];

    println!("The first two months are: {}, {}", first, second);

}

pub fn match_num_4(){
    let number = 6;

    match number % 4 {
        0 => println!("number is divisible by 4"),
        _ => println!("number is not divisible by 4"),
    }
}

pub fn all_data_types(){
    variable_and_constant();
    shadowing_example();
    data_type();
    compound_types();
    match_num_4();
}