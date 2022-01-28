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
    )
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
