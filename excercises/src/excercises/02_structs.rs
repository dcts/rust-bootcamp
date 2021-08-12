pub fn run() {
    println!("=== STRUCTS ===");
    println!("generate person:");
    let mut p: Person = Person::new("frakie","miller", 30);
    p.print();
    p.happy_birthday();
    p.happy_birthday();
    p.print();
    p.set_last_name("Rogan");
    p.print();
    println!("p.to_string() : {}", p.to_string());
}

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}
impl Person {
    fn new(first_name: &str, last_name: &str, age: u8) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age: age,
        }
    }
    fn print(&self) {
        println!("{} {} is {} years old.", self.first_name, self.last_name, self.age);
    }
    fn to_string(&self) -> String {
        format!("{} {} is {} years old.", self.first_name, self.last_name, self.age)
    }
    fn happy_birthday(&mut self) {
        self.age += 1;
    }
    fn set_last_name(&mut self, new_last_name: &str) {
        self.last_name = new_last_name.to_string();
    }
}
