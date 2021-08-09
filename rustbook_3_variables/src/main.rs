fn main() {
    println!("=== Celsius to Fahrenheit ===");
    let mut celsius: i32 = -30;
    println!("cel: {} => {} fahrenheit", celsius, convert_to_fahrenheit(celsius));
    celsius = -10;
    println!("cel: {} => {} fahrenheit", celsius, convert_to_fahrenheit(celsius));
    celsius = 0;
    println!("cel: {} => {} fahrenheit", celsius, convert_to_fahrenheit(celsius));
    celsius = 20;
    println!("cel: {} => {} fahrenheit", celsius, convert_to_fahrenheit(celsius));
    celsius = 30;
    println!("cel: {} => {} fahrenheit", celsius, convert_to_fahrenheit(celsius));

    println!("\n\n=== Fibonacci Numbers ===");
    for x in 1..=100 {
        println!("fibonacci {}: {}", x, fibonacci(x));
    }
}

fn convert_to_fahrenheit(celsius: i32) -> i32 {
    celsius * (9/5) - 32
}

fn fibonacci(n: u128) -> u128 {
    let mut sequence: Vec<u128> = vec![0,1];
    while (sequence.len() as u128) < n + 1 {
        let len: usize = sequence.len();
        sequence.push(sequence[len - 1] + sequence[len - 2]);
    }
    sequence[(n - 1) as usize]
}
