pub fn run() {
    println!("=== LOOPS ===");

    // Infinite loop
    let mut count: i8 = 0;
    loop {
        count += 1;
        println!("Iteration: {}", count);

        if count == 20 {
            break;
        }
    }

    // while loop (FizzBuzz)
    count = 0;
    println!("\nFIZZBUZZ Excercise");
    while count <= 100 {
        if count % 5 == 0 && count % 3 == 0 {
            println!("nbr_{}: FizzBuzz", count);
        } else if count % 5 == 0 {
            println!("nbr_{}: Buzz", count);
        } else if count % 3 == 0 {
            println!("nbr_{}: Fizz", count);
        } else {
            println!("nbr_{}:", count);
        }
        count += 1;
    }

    // for range
    println!("\nfor x in 1..10 (NOT including 10)");
    for x in 1..10 {
        println!("{}", x);
    }
    println!("\nfor x in 1..=10 (including 10)");
    for x in 1..=10 {
        println!("{}", x);
    }
}
