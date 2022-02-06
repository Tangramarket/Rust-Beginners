const NUM: i32 = 90;

struct Member {
    // create a struct
    id: i32, // member : type i32
    name: String,
    working: bool,
}

struct Square {
    // create a struct
    len: i32,
    wid: i32,
}

enum Language {
    // define an enum
    JS, // member
    GO,
    VB,
}

mod my_module {
    // define a module
    pub fn test() {
        // pub means public attribute
        println!("Hello, My Friends! Reuslt from my_module");
    }
}

mod mod_with_emb {
    pub fn a() {
        println!("Result from mod_with_emb Module");
    }

    pub mod emb_module {
        // embedded module, public
        pub fn b() {
            println!("Result from emb_module");
        }
    }
}

// Load the external file and function
mod ext_file_test; // loads an external file
use ext_file_test::ext_fun; //loads an external function

// Private Function
mod my_mod_private_fun {
    pub fn public_a() {
        // function is public
        println!("Public Function - public_a");
    }

    /*
    // Function is never used, Therefore an error occurred
    fn private_b() {
        println!("Private Function - private_b");
    }
    */

    pub fn public_a2() {
        println!("Public Function - public_a2");
        private_b2(); //Call a private function private_b2, no error occurred
    }

    fn private_b2() {
        println!("PRIVATE Function - public_b2");
    }
}

// Super
mod super_module {
    // Parent Module
    fn a_parent() -> i32 {
        100 // without ;
    }

    pub mod sub_module {
        // Child module
        use super::a_parent; // access parent function a_super
        pub fn b_child() {
            println!("{}", a_parent()); // Calls parent function a_parent
        }
    }
}

//Method
struct Circle {
    // create a struct type
    radius: f32, // struct member
}

impl Circle {
    // implement the struct
    fn area(&self) -> f32 {
        // define a method
        std::f32::consts::PI * self.radius * self.radius
    } // method body
}

// Trait
struct CircleTrait {
    // create a struct type
    radius: f32, // struct member
}

trait Calculate {
    // define a trait
    fn area(&self) -> f32; // define a trait method
}

impl Calculate for CircleTrait {
    // implement the trait
    fn area(&self) -> f32 {
        // implement the trait method
        std::f32::consts::PI * self.radius * self.radius
    }
}

// Drop() Method - LIFO
struct Game {
    number: i32,
}

impl Drop for Game {
    fn drop(&mut self) {
        // define a drop method
        println!("The #{ } Winner . ", self.number);
    }
}

// fn main() Start
fn main() {
    println!();
    println!("Hello, Beginner!");
    let var = "OK";
    println!("var - The value of var is: {}", var);

    //fn foo()
    println!();
    println!("-- foo() is a caller --");
    let b = foo(); //foo() is a caller
    println!("fn foo() - The result is: {}", b);

    // fun2()
    println!();
    println!("-- calls the function --");
    let num = funt2(100); //calls the function
    println!("fun2() - The value of num is: {}", num);

    //const
    println!();
    println!("-- const --");
    println!("const - The value of NUM is {}", NUM);
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
    println!("-- Calls the function --");
    funt(100, 200); // calls the function

    // mutable variables
    println!("-- Mutable variabless --");
    let mut aa = 100; // variable-binding
    let mut bb = 200;
    aa = aa + 300;
    bb = bb + 400;
    println!("Finally a is {}", aa);
    println!("Finally b is {}", bb);

    //String Assignment - 3 methods
    println!();
    println!("-- String Assignment - 3 methods --");
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

    //Struct
    // Outside fn main()
    /*
    struct Member {
    // create a struct
    id: i32, // member : type i32
    name: String,
    working: bool,
    } */
    println!();
    println!("-- Struct --");
    let clerk = Member {
        // initialize the struct
        id: 016320, // member : value
        name: "Smith".to_string(),
        working: true,
    };
    println!("ID is {}", clerk.id); // access the members
    println!("Name is {}", clerk.name);
    println!("Working is {}", clerk.working);
    println!();

    /*
    struct Square {
    // create a struct, Outside fn main()
    len: i32,
    wid: i32,
    }
    */
    let table = Square { len: 10, wid: 8 }; // 	initialization
    println!("The area is {}", table.len * table.wid); // access

    //Enum. Enumeration is a custom data type that contains certain values
    /* Outside fn main()
    enum Language {
    // define an enum
    JS, // member
    GO,
    VB,
    }
    */
    // fn program(var: Language) - Outside fn main()
    println!();
    println!("-- Enum --");
    program(Language::JS); // access the member
    program(Language::GO);
    program(Language::VB);

    /*
    //Ownership
    println!();
    println!("-- Ownership --");
    let _old_owner = String::from("try"); // old_owner owns “try”
    let _new_owner = _old_owner; // Warning ! The ownership of old_owner moves to new_owner
    println!("{}", _old_owner); // Error ! old_owner is no longer available
                                /* For more information about this error, try `rustc --explain E0382`.
                                error: could not compile `rust-basic` due to previous error */


    let s =String :: from("R in 8 Hours"); // s owns “R in 8 Hours”
    let n=cal( s ); // Warning ! s will lose the ownership after used
    println ! ("Value of the string is : {}", s ); // s is no longer available
    println ! ("Length of the string is : {}",n);
    */

    //Reference
    println!();
    println!("-- Reference --");
    let _s = String::from("R in 8 Hours");
    let _n = cal_ref(&_s); // reference
    println!("Value of the string is : {}", _s);
    println!("Length of the string is : {}", _n);

    //Module
    println!();
    println!("-- Module --");
    my_module::test(); // run the module

    //Embedded Module
    println!();
    println!("-- Embedded Module --");
    mod_with_emb::a();
    mod_with_emb::emb_module::b(); //runs the embedded module & function

    //External File
    println!();
    println!("-- External File --");
    ext_fun(); // calls the external function

    //Private Function
    println!();
    println!("-- Private Function --");
    my_mod_private_fun::public_a();
    /* Calls function private_b from outside the module . Therefore an error occurred .
    my_mod_private_fun::private_b();
    */

    // Calls function private_b2 from public_a2, No error occurred
    println!();
    println!("-- Private Function - No error occurred --");
    my_mod_private_fun::public_a2();

    //Super
    println!();
    println!("-- Super --");
    super_module::sub_module::b_child(); // call function b_child

    //Vector
    println!();
    println!("-- Vector 1st method --");
    let v1 = vec![100, 200, 300, 400]; // create a vector
    println!("First element is : {}", v1[0]); // access the first element
    println!("Second element is : {}", v1[1]);
    println!("Third element is : {}", v1[2]);
    println!("Fourth element is : {}", v1[3]);

    println!();
    println!("-- Vector 2nd method --");
    let v2 = vec![10; 3]; // repeat three times, also let v2 = vec!["a"; 3];
    println!("First element is : {}", v2[0]);
    println!("Second element is : {}", v2[1]);
    println!("Third element is : {}", v2[2]);

    println!();
    println!("-- Vector 3rd method --");
    let mut v3 = Vec::new(); // create a vector
    v3.push('L'); // set L as the first element of vector
    v3.push('O');
    v3.push('N');
    v3.push('D');
    v3.push('O');
    v3.push('N');
    for n in v3 {
        print!("{}", n);
    }

    // Multiple Patterns
    println!();
    println!();
    println!("-- Multiple Patterns - use | to match multiple patterns --");
    let patt_num = 3;
    match patt_num {
        1 => println!("one"),
        2 | 3 => println!("two or three"), // multiple patterns
        _ => println!("others"),
    }

    // Range
    println!();
    println!("-- Range --");
    let range_x = 4;
    match range_x {
        2..=8 => println!("from 2 to 8"), // match from 2 to 8
        _ => println!("others"),
    }

    // Binding a Range
    println!();
    println!("-- Binding a Range --");
    let bindin_y = 3;
    match bindin_y {
        var @ 2..=9 => println!("{}", var), // binding, print bindin_y
        _ => println!("others"),
    }

    // Generics
    println!();
    println!("-- Generics --");
    let gen_x: Option<bool> = Some(false); // generic parameters
    let gen_y: Option<i32> = Some(22);
    let gen_z: Option<f64> = Some(45.47);
    let gen_m: Option<i32> = None;
    match gen_x {
        Some(gen_x) => {
            println!("gen_x = {}", gen_x)
        }
        None => {
            println!("gen_x = None")
        }
    }
    match gen_y {
        Some(gen_y) => {
            println!("gen_y = {}", gen_y)
        }
        None => {
            println!("gen_y = None")
        }
    }
    match gen_z {
        Some(gen_z) => {
            println!("gen_z = {}", gen_z)
        }
        None => {
            println!("gen_z = None")
        }
    }
    match gen_m {
        Some(gen_m) => {
            println!("gen_m = {}", gen_m)
        }
        None => {
            println!("gen_m = None")
        }
    }

    // Method
    println!();
    println!("-- Method --");
    let obj = Circle { radius: 2000.00 }; // create a struct object
    println!("The Circle area is : {}", obj.area()); // call the method

    // Trait, interface in Rust
    println!();
    println!("-- Trait --");
    let obj = CircleTrait { radius: 2000.00 }; // create a struct object
    println!("The Circle area is : {}", obj.area()); // call the method

    // Closure
    println!();
    println!("-- Closure --");
    let my_closure = |num: i32| num + 200; // create a closure
    let num = 100;
    println!("{}", my_closure(num)); // call the closure

    println!();
    println!("-- Closure --");
    let mut capacity = "Hard disk capacity : 5000".to_string();
    {
        let mut my_closure = |c: char| capacity.push(c); // closure
        my_closure('G'); // call the closure
    }
    println!("{:?}", capacity); // { : ?} is used to output a string

    // Error Checking
    println!();
    println!("-- Error Checking --");
    let check: bool = true; // suppose it is true
    assert!(check == true); // check the error
    println!("{}", check);

    // Drop() Method - LIFO
    println!();
    println!("-- Drop --");
    let _baseball = Game { number: 3 };
    let _football = Game { number: 2 };
    let _basketball = Game { number: 1 };
} // end fn main()

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

// for Enum example
fn program(var: Language) {
    match var {
        // using match statement
        Language::JS => println!("JS in 8 Hours"),
        Language::GO => println!("GO in 8 Hours"),
        Language::VB => println!("VB in 8 Hours"),
    }
}

/*
//with Ownership example
fn cal (s : String) -> usize {
    s . len() // get the length of the string
}
*/

//Reference
fn cal_ref(s: &String) -> usize {
    // reference
    s.len() // get the length of the string
}
