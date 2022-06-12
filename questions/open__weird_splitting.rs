pub fn main() {
    // weird splitting behaviour
    println!("\nWEIRD SPLITTING BEHAVIOUR (why are first and last char empty?)");
    let str = String::from("Hello World");
    let mut indx: u8 = 0;
    for char in str.split("") {
        println!("index {}: '{}'", indx, char);
        indx += 1;
    }
}
