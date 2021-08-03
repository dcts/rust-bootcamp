// Structs => used to create custom data types
// similar to classes

pub fn run() {
    println!("=== STRUCTS ===");

    // initialize an instance of Color
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };
    // change value
    println!("\nTRADITIONAL STRUCT:\nbefore change: color: {},{},{}", c.red, c.green, c.blue);
    c.red = 200;
    println!("after color  : color: {},{},{}", c.red, c.green, c.blue);

    // initialize an instance of Color (tuple struct)
    let mut c2 = ColorSimple(0,255,0);
    println!("\nTUPLE STRUCT:\nbefore change: color: {},{},{}", c2.0, c2.1, c2.2);
    c2.1 = 200;
    println!("after change : color: {},{},{}", c2.0, c2.1, c2.2);

    // Initialize Person
    let mut p = Person::new("Frankie", "Miller", 30);
    println!("\nSTRUCTS\n{:?}", (&p.first_name, &p.last_name, &p.age));
    // use "&" reference because otherwise the ownership goes to println macro
    println!("Person: {}", p.fullname());
    p.print();
    p.happy_birthday();
    p.print();
    p.happy_birthday();
    p.print();
    p.happy_birthday();
    p.print();
    println!("{:?}", p.to_tuple());
}

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct ColorSimple(u8, u8, u8);

// if you want to be able to print the struct to the console
// with the debug trait "{:?}", then you need to add this line
// on top of the enum declaration
#[derive(Debug)]
// Struct with functions
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}
impl Person {
    fn new(first: &str, last: &str, age: u8) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
            age: age,
        }
    }
    // print person inf
    fn print(&self) {
        println!("{} {} is {} years old.", self.first_name, self.last_name, self.age);
    }
    // get full name
    fn fullname(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    // set age
    fn happy_birthday(&mut self) {
        self.age += 1;
    }
    fn to_tuple(self) -> (String, String, u8) {
        (self.first_name, self.last_name, self.age)
    }
}

