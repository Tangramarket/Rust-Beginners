const NUM: i32 = 90;

fn main() {
    //fn foo()
    println!();
    let b = foo(); //foo() is a caller
    println!("fn foo() - The result is: {}", b);

    // fun2()
    println!();
    let num = funt2(100); //calls the function
    println!("fun2() - The value of num is: {}", num);

    //const
    println!();
    println!("const - The value of NUM is {}", NUM);
    println!();

    println!("Hello, Beginner!");
    let var = "OK";
    println!("var - The value of var is: {}", var);
    println!();

    let var0: i32 = 100; // defines the type of var as i32
    let str: String = "Good".to_string(); // . to_string()” converts the value to String type
    println!("The value of var2 is: {}", var0);
    println!("The value of str is: {}", str);
    println!();

    let x = 100;
    let y = 200;
    let z = 300;
    println!("{}", x); // new line
    print!("{} {}", y, z); // the same line
    println!();

    let var1: f32 = 100.88;
    let var2: i32 = var1 as i32; //var1 becomes i32 - integer
    println!("{}", var1);
    println!("{}", var2);
    println!();

    // funt
    funt(100, 200); // calls the function

    // mutable variables
    let mut aa = 100; // variable-binding
    let mut bb = 200;
    aa = aa + 300;
    bb = bb + 400;
    println!("Finally a is {}", aa);
    println!("Finally b is {}", bb);

    //String Assignment - 3 methods
    println!();
    let x_str = "hello".to_string(); // convert text to a string
    let y_str = String::from("hello"); // get text directly
    let z_str: &str = "hello"; // reference a text
    print!("{} {} {} ", x_str, y_str, z_str);
    println!();

    //Arithmetical Operators
    println!();
    println!("-- Arithmetical Operators --");
    println!("10 + 2 = {}", 10 + 2);
    println!("10 - 2 = {}", 10 - 2);
    println!("10 * 2 = {}", 10 * 2);
    println!("10 / 2 = {}", 10 / 2);
    println!("10 % 2 = {}", 10 % 2); // modulus operator divides the 1st by the 2nd number and returns the remainder

    //Logical Operators
    println!();
    println!("-- Logical Operators --");
    println!("true AND false is {}", true && false); // and &&
    println!("true OR false is {}", true || false); // or ||
    println!("NOT true is {}", !true); // not !

    //Comparison Operators
    println!();
    println!("-- Comparison Operators --");
    let _xx: i32 = 1000;
    let _yy: i32 = 2000;
    println!("x is greater than y: {}", _xx > _yy);
    println!("x is less than y: {}", _xx < _yy);
    println!("x is unequal to y: {}", _xx != _yy); // != without space
    println!("x is greater/equal to y: {}", _xx >= _yy);
    println!("x is less/equal to y: {}", _xx <= _yy);
    println!("x is completely equal to y: {}", _xx == _yy);

    //Arrays
    println!();
    println!("-- Arrays --");
    // The first way
    let mut a_array1: [i32; 4] = [100; 4]; // create an array type i32, 4 elements, default value is 100
    a_array1[1] = 77;
    a_array1[2] = 88;
    println!(
        "{} {} {} {}",
        a_array1[0],
        a_array1[1],
        a_array1[2],
        a_array1[3] // [0] and [3] are with default values = 100
    );

    // The second way
    println!();
    let a_array2: [f32; 4] = [0.1, 0.2, 0.3, 0.4]; // create an	array secon way
    println!(
        "{} {} {} {}",
        a_array2[0], a_array2[1], a_array2[2], a_array2[3]
    );

    //Slice
    println!();
    println!("-- Slice --");
    let array3 = [0, 10, 20, 30, 40, 50, 60]; // create an array
    let slice = &array3[2..5]; /* extract three elements from a[2] to a[4], now the slice contains three elements */
    println!("{}", slice[0]);
    println!("{}", slice[1]);
    println!("{}", slice[2]);

    //If Statement
    println!();
    println!("-- If Statement --");
    let number = 10;
    if number == 10 {
        println!("num is equal to 10");
    }

    //If-else Statement
    println!();
    println!("-- If-else Statement --");
    let num1 = 99;
    let num2 = 888;
    if num1 > num2 {
        println!("num1 is greater than num2");
    } else {
        println!("num1 is smaller than num2");
    }

    //Let-If Statement
    println!();
    println!("-- Let-If Statement --");
    let age = 10; // >= 13  < 20 for True
    let is_teenager = if age < 20 {
        if age >= 13 {
            true
        } else {
            false
        }
    } else {
        false
    };
    println!("Is teenager: {}", is_teenager);

    //Loop – Break Statement
    println!();
    println!("-- Loop – Break Statement --");
    let mut hours = 5;
    loop {
        // loop statement
        println!("Rust in {} Hours", hours); // from 5 to 8
        if hours == 8 {
            break; // break statement
        }
        hours = hours + 1;
    }

    //For Statement
    println!();
    println!("-- For Statement --");
    for _hours in 5..9 {
        // “5 .. 9” contains numbers from 5 to 8
        println!("Java in {} Hours", _hours); // from 5 to 8 only!
    }

    //While Statement
    println!();
    println!("-- While Statement --");
    let mut while_num = 0;
    while while_num <= 8 {
        // while statement
        print!("{} ", while_num); // 0 1 2 3 4 5 6 7 8
        while_num = while_num + 1;
    }
    println!();

    //Tuples
    println!();
    println!("-- Tuples --");
    let t = ("Python in", 8, "Hours", true); // create a tuple
    print!("{} {} {} {}", t.0, t.1, t.2, t.3); // access the elements

    //Match
    println!();
    println!("-- Match --");
    let num_match: i32 = 3; // given expression
    match num_match {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"), // match this
        4 => println!("four"),
        _ => println!("something else"), // if don't match above values, run this
    }
}

fn foo() -> bool {
    // specify a return type
    return true; // return a value to the caller, without ;
}

fn funt2(num: i32) -> i32 {
    // specify a return type
    num + 200 // return a value to the caller, without ;
}

fn funt(x: i32, y: i32) {
    // define a function
    println!("funt() - The sum is: {}", x + y);
}
