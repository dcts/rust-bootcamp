// tuples group together values of different types
// max 12 elements

pub fn run() {
    println!("=== TUPLES ===");

    // example tuple
    let person: (&str, &str, i8) = ("thomas", "berlin", 30);
    println!("{} is from {} and is {}", person.0, person.1, person.2);

    // demonstrate max elements
    let integers = (1,2,3,4,5,6,7,8,9,10,11,12);
    println!("{:?}", (integers.0, integers.11));
    println!("{:?}", integers);
}
