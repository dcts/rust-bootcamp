// Enums are types that have a few definite values (called "Variants")
pub fn run() {
    println!("=== ENUMS ===");
    move_avatar(Movement::Up);
    move_avatar(Movement::Down);
    move_avatar(Movement::Left);
    move_avatar(Movement::Right);
}

// if you want to be able to print the value to the console
// with the debug trait "{:?}", then you need to add this line
// on top of the enum declaration
#[derive(Debug)]
enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    // perform action depending on Movement
    match m {
        Movement::Up => println!("Avatar moving 👆"),
        Movement::Down => println!("Avatar moving 👇"),
        Movement::Right => println!("Avatar moving 👉"),
        Movement::Left => println!("Avatar moving 👈"),
    }
}
