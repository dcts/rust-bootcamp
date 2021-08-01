// Primitive Strings => fixed length, immutable, stored somewhere in memory
// String = Growable, heap type data structure. Use when needed to modify

pub fn run() {
    println!("=== STRINGS ===");

    // primitive strings
    let str_1 = "my primitive string";
    println!("This is a primitive string: {}, len: {}", str_1, str_1.len());

    // growable strings
    let mut str_2 = String::from("Hello ");

    // push to string (APPEND)
    str_2.push('W'); // push character
    str_2.push_str("orld"); // push string
    println!("String: {}, len: {}", str_2, str_2.len());

    // useful functions
    println!("Capacity: {}", str_2.capacity()); // capacity in bytes
    println!("Is empty? {}", str_2.is_empty()); // is empty? ""
    println!("Contains 'World': {}", str_2.contains("World"));
    println!("Contains 'World': {}", str_2.contains("Tulipe"));
    println!("Replace 'World' with 'There': {}", str_2.replace("World", "There"));
    println!("State of str_2: {}", str_2);
    println!("SPLIT BY WHITESPACE");
    let mut count = 0;
    for word in str_2.chars() {
        println!("Word {}: {}", count, word);
        count += 1;
    }

    // Create string with certain capacity
    let capacity: usize = 10;
    let mut s = String::with_capacity(capacity);
    s.push('a');
    s.push('b');
    println!("CREATE STRING WITH CAPACITY {}: \ns = '{}'", capacity, s);

    // Assertion testiong
    assert_eq!(s.len(), 2);
    assert_eq!(s.capacity(), capacity);
}
