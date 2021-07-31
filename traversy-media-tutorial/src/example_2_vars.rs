// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "frankie";
    let mut age = 30;

    println!("my name is {}, and I am {} years old.", name, age);
    age = 31;
    println!("Now I am {} years old.", age);

    // define constants (have to be UPCASED)
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple variables at once
    let ( my_name, my_age ) = ("Frankie", 30);
    println!("{} is {} years old.", my_name, my_age);
}
