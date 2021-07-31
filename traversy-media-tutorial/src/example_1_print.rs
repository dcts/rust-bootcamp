pub fn run() {
    // Print to console
    println!("Hello from the example_1_print.rs file");

    // Interpolation
    println!("Number: {}", 12);
    println!("String: {}", "Hello Rustians");

    // Positional arguments
    let first_name = "frankie";
    let last_name = "müller";
    let hobbies = ["code", "workout"];
    println!("{0} {1} loves to {2} and {0} also loves to {3}", first_name, last_name, hobbies[0], hobbies[1]);

    // Named arguments
    println!(
        "{name} likes to {hobby}",
        name = "John",
        hobby = "code",
    );

    // placeholder traits
    println!("Binary: {:b}. Hex: {:x}. Octo: {:o}", 10, 10, 10);

    // placeholder for debug traits
    println!("{:?}", (12,  true, "Hello World"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}
