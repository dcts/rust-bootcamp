pub fn run() {
    println!("=== FUNCTIONS ===");
    // EXCERCISE 1.A reverse string
    println!("=== reverse_original_string ===");
    let mut s: String = String::from("Hello World");
    println!("s (before call): {}", s);
    reverse_original_string(&mut s);
    println!("s (after call) : {}", s);

    println!("\n=== reverse_string_drop_old_value ===");
    let s2: String = String::from("Hello World");
    println!("s2 (before call)   : {}", s2);
    let s2_new: String = reverse_string_drop_old_value(s2);
    println!("s2 (after call)    : 🚧 will not compile, because value was moved (=dropped)");
    println!("s2_new (after call): {}", s2_new);

    println!("\n=== reverse_string_by_cloning_str_slice ===");
    let s3: String = "Hello World".to_string();
    println!("s3 (before call)   : {}", s3);
    let s3_str_slice = &s3[..];
    let s3_new: String = reverse_string_by_cloning_str_slice(s3_str_slice);
    println!("s3 (after call)    : {}", s3);
    println!("s3_new (after call): {}", s3_new);

    // EXCERCISE 1.B reverse string
    println!("\n=== MAX ===");
    let (x1, x2) = (-122387,124);
    println!("max of {} and {} is {}", x1, x2, max(x1,x2));
    let (x1, x2) = (-125,-2);
    println!("max of {} and {} is {}", x1, x2, max(x1,x2));
    let (x1, x2) = (142,2);
    println!("max of {} and {} is {}", x1, x2, max(x1,x2));
}

/*
 * MUTATES ORIGINAL STRING, does not return anything
 * => hence we need to pass in a mutable reference
 * => reverse_original_string has borrowed the ownership of the string variable
 * => it does not return anything, and all memory that was used for the
 *    reversing is freed after the function reaches it's end (remember,
 *    rust has no garbage collection)
 * => uses the dereferencing operator `*`
 *    (Dereferencing is used to access the data of the reference and reassign it!)
 *    (the &mut only means that the reference is mutable, so the data can change)
 */
fn reverse_original_string(s: &mut String) {
    *s = s.chars().rev().collect::<String>();
}

/*
 * TAKES OWNERSHIP, RETURNS NEW VALUE
 * => be aware, the original string is dropped after this functions ends
 */
fn reverse_string_drop_old_value(s: String) -> String {
    s.chars().rev().collect::<String>()
}
/*
 * TAKES AN IMMUTABLE STRING SLICE
 * => clones it, reverses it and returns a new string
 */
fn reverse_string_by_cloning_str_slice(s: &str) -> String {
    s.clone().chars().rev().collect::<String>()
}

/*
 * COMPARISON Rust with Javascript
 */
// JS   => https://stackoverflow.com/questions/20740513/does-reassigning-a-javascript-variable-empty-its-previous-value-from-memory-righ
//      => no control over memory assignment
//      => garbage collector
// RUST => Memory is direclty freed.
//      => Developer can reassign memory with shadowwing, which is not possible in JS

/*
 * MAX OF 2 NUMBERS
 * => return new value
 */
fn max(x: i32, y: i32) -> i32 {
    if x > y { x } else { y }
}
