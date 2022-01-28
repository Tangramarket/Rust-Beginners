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
    println!("10 + 2 = {}", 10 + 2);
    println!("10 - 2 = {}", 10 - 2);
    println!("10 * 2 = {}", 10 * 2);
    println!("10 / 2 = {}", 10 / 2);
    println!("10 % 2 = {}", 10 % 2); // modulus operator divides the 1st by the 2nd number and returns the remainder

    //Logical Operators
    println!();
    println!("true AND false is {}", true && false); // and &&
    println!("true OR false is {}", true || false); // or ||
    println!("NOT true is {}", !true); // not !

    //Comparison Operators
    println!();
    let _xx: i32 = 1000;
    let _yy: i32 = 2000;
    println!("x is greater than y: {}", x > y);
    println!("x is less than y: {}", x < y);
    println!("x is unequal to y: {}", x != y); // != without space
    println!("x is greater/equal to y: {}", x >= y);
    println!("x is less/equal to y: {}", x <= y);
    println!("x is completely equal to y: {}", x == y);
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
