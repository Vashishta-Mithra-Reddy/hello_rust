use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut name = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter first number:");
    io::stdin().read_line(&mut num1).expect("Failed to read input");

    println!("Enter second number:");
    io::stdin().read_line(&mut num2).expect("Failed to read input");

    let num1: i32 = num1.trim().parse().expect("Please enter a valid number");
    let num2: i32 = num2.trim().parse().expect("Please enter a valid number");

    println!("");
    print!("Your name is: {}",name);
    println!("Sum: {}", add(num1, num2));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
