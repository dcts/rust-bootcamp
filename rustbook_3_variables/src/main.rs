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
}

fn convert_to_fahrenheit(celsius: i32) -> i32 {
    celsius * (9/5) - 32
}
