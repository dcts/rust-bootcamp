pub fn run() {
    println!("=== COLLATZ CONJECTURE (3x+1) ===");
    println!("{:?}", collatz(5));  // results in 4,2,1 end loop
    println!("{:?}", collatz(12)); // results in 4,2,1 end loop
    println!("{:?}", collatz(-5)); // results in a negative loop

    println!("\n=== COLLATZ CONJECTURE RECURSIVE (3x+1) ===");
    for x in -10..=10 {
        let mut list: Vec<i64> = Vec::new();
        collatz_recursive(x, &mut list);
        println!("collatz of {}: {:?}", x, &mut list);
    }

    println!("\n\nTESTING: \n");
    println!("{:?}", collatz(4));
    println!("{:?}", collatz(5));
    println!("{:?}", collatz(6));
    println!("{:?}", collatz(7));
    println!("{:?}", collatz(8));
    println!("{:?}", collatz(10));
    println!("{:?}", collatz(20));
}

/*
 * COLLATZ WITH LOOP
 */
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


/*
 * COLLATZ WITH RECURSION
 * Example recursive function taken from here:
 * https://www.programming-idioms.org/idiom/31/recursive-factorial-simple/450/rust
 */
// fn factorial(num: u64) -> u64 {
//     match num {
//         0 | 1 => 1,
//         _ => factorial(num - 1) * num,
//     }
// }
fn collatz_recursive(num: i64, list: &mut Vec<i64>) {
    let exists: bool = list.contains(&num);
    match exists {
        true => {}
        false => {
            list.push(num);
            let even: bool = num % 2 == 0;
            match even {
                true => collatz_recursive(num / 2, list),
                false => collatz_recursive(3 * num + 1, list),
            }
        }
    }
}
