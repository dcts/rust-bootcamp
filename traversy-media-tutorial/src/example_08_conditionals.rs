use std::io;

pub fn run() {
    println!("=== CONDITIONALS ===");

    println!("How old are you? Enter age: ");
    let mut age = String::new();

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read input");

    let age: u8 = match age.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    // If/Else
    if age > 100 {
        println!("[Age:{}] => grandpa, go home 👴", age);
    } else if age >= 18 {
        println!("[Age:{}] => can drink! 🍻", age);
    } else {
        println!("[Age:{}] => cannot drink! ❌", age);
    }

    // If/Else shorthand
    let can_drink: bool = if age > 18 { true } else { false };
    println!("can_drink = {}", can_drink);

    // 🔄 FLOW / CONDITIONALS
    // - and => &&
    // - or  => ||

    // ❌ NO TERNERY OPERATOR
}
