fn main() {
    let _a = "Immutable string!"; // prefixing with "_" as it's an unused variable

    // _a = "Changing an immutable string... "; // this throws a compiler error

    let _a = "Shadowing an immutable string!";

    // Scalar Data Types

    const _PEP8_COLUMN_MAX_LENGTH: u32 = 80;

    let _a: i32 = 42; // this is the same as not specifying the type

    let _a_float: f64 = 42.42;

    let _a: bool = true;

    let _a: char = 'a';

    // Compound Data Types

    let _a_tuple: (u32, i32, f64) = (500, -500, 500.500);

    let _a_index_0 = _a_tuple.0;

    let _a_array = [1, 2, 3, 4, 5];

    let _a_index_0 = _a_array[0];

    function_1();

    function_2(-50.50);

    //this is a statement
    let _y = 50;

    // statements don't have a return value
    // let y = (let y = 50);

    let _y = {
        5 // adding a semicolon here will turn the block into an statement!
    };

    println!("{}", _y);

    // there is  no syntax for
    // multiple line comments

    if true {
        println!("Yup!")
    } else {
        println!("Nope...")
    }

    if function_3() == 100 {
        println!("The value was 100.")
    }

    if (function_3() - 1) % 2 == 0 {
        println!("Divisible by 2.")
    } else if (function_3() - 1) % 3 == 0 {
        println!("Divisible by 3.")
    }

    if false {

    } else {
        println!("False.")
    }

    for x in (1..6).rev() {
        println!("{}...", x);
    }

    let mut x = 5;

    while x != 0 {
        println!("{}...", x);

        x = x - 1;
    }

    for ele in _a_array.iter() {
        println!("{}...", ele);
    }   

    // loop {
    //     println!("Printing forever...")
    // }
}

fn function_1() {
    println!("Printing from function_1!");
}

fn function_2(x: f64) {
    println!("Printing from function_2 with {}.", x);
}

fn function_3() -> u32 {
    100
}