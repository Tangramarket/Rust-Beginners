const NUM:i32 = 100;

fn funt(x: i32, y: i32) { // define a function
	println!("funt() - The sum is: {}", x + y);
}

fn funt2(num: i32) -> i32{ // specify a return type
	num + 200 // return a value to the caller, without ;
}

fn foo() -> bool { // specify a return type
	return true // return a value to the caller, without ;
}

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
	let str:String = "Good".to_string(); // . to_string()” converts the value to String type
	println!("The value of var2 is: {}", var0);
	println!("The value of str is: {}", str);
	println!();
	
	let x = 100;
	let y = 200;
	let z = 300;
	println!("{}", x);
	print!("{} {}", y, z);
	println!();
	
	let var1:f32 = 100.88;
	let var2:i32 = var1 as i32; //var1 becomes i32 - integer
	println!("{}", var1);
	println!("{}", var2);
	println!();
	
	// funt
	funt(100, 200); // calls the function
	
}

