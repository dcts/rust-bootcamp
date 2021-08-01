// Functions - used to store blocks of code to re-use

pub fn run() {
    println!("=== FUNCTIONS ===");
    // greet user
    greeting("hello", "frakie");

    // compute sum
    let a = 12;
    let b = 243;
    let sum_of_numbers: i32 = add(a, b);
    println!("{} + {} = {}", a, b, sum_of_numbers);

    // closure (allows using outside variables)
    let x3: i32 = 1000;
    let add_nums = |x1: i32, x2: i32| x1 + x2 + x3;
    println!("add_nums => {}", add_nums(12,23));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

// DO NOT USE A SEMICOLON => indicating the
// program that this is the return value!
fn add(x1: i32, x2: i32) -> i32 {
    x1 + x2
}
