use std::env;

pub fn run() {
    println!("=== CLI Interface ===");

    let args: Vec<String> = env::args().collect();
    println!("SHOW ARGS: {:?}", args);

    // take first argument as command
    let command = args[1].clone();
    println!("COMMAND: {}\n", command);

    if command == "greet" {
        println!("👋 Hello {}", args[2].clone());
    } else if command == "add" {
        let num1: i32 = args[2].trim().parse().expect("Failed to parse input");
        let num2: i32 = args[3].trim().parse().expect("Failed to parse input");
        let result: i32 = num1 + num2;
        println!("{} + {} = {}", num1, num2, result);
    } else if command == "substract" {
        // let num1: i32 = args[2].trim().parse().expect(format!("Failed to parse input: {}", args[2]));
        let num1: i32 = args[2].trim().parse().expect("Failed to parse input");
        let num2: i32 = args[3].trim().parse().expect("Failed to parse input");
        let result: i32 = num1 - num2;
        println!("{} + {} = {}", num1, num2, result);
    }
}
