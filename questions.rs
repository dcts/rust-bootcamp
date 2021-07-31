pub fn main() {
    // floats not displayed properly
    println!("\nFLOATS ARE NOT DISPLAYED PROPERLY");
    println!("f32: {} to {}", std::f32::MIN, std::f32::MAX);
    println!("f64: {} to {}", std::f64::MIN, std::f64::MAX);

    // weird splitting behaviour
    println!("\nWEIRD SPLITTING BEHAVIOUR (why are first and last char empty?)");
    let str = String::from("Hello World");
    let mut indx: u8 = 0;
    for char in str.split("") {
        println!("index {}: '{}'", indx, char);
        indx += 1;
    }

    // tuples only 12 max items, never specified
    // https://stackoverflow.com/q/51846320/6272061
    let short = (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    println!("\n{:?}", short); // Works fine
    // let long = (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    // println!("{:?}", long); // DOES NOT WORK
}
