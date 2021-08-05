pub fn run() {
    println!("=== COLLATZ CONJECTURE (3x+1) ===");
    println!("{:?}", collatz(5));  // results in 4,2,1 end loop
    println!("{:?}", collatz(12)); // results in 4,2,1 end loop
    println!("{:?}", collatz(-5)); // results in a negative loop
}

fn collatz(seed: i32) -> Vec<i32> {
    let mut input: i32 = seed;
    // initialize sequence with seed
    let mut sequence: Vec<i32> = vec![input];
    let mut next_number: i32 = collatz_next_number(input);
    while !sequence.contains(&next_number) {
        sequence.push(next_number);
        input = next_number;
        next_number = collatz_next_number(input);
    }
    sequence
}

fn collatz_next_number(input: i32) -> i32 {
    let next_number: i32;
    if input % 2 == 0 { // if input is even
        next_number =  input / 2;
    } else { // if number is odd
        next_number =  input * 3 + 1;
    }
    next_number
}
