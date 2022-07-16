
pub fn run() {
    println!("=== OPTIONS ===");

    // get an option like this
    let maybe_marrys_occupation = get_occupation("marry");
    let maybe_asdjkhk_occupation = get_occupation("asdjkhk");
    println!("marry's occupation is: {:?}", maybe_marrys_occupation);
    println!("asdjkhk's occupation is: {:?}", maybe_asdjkhk_occupation);

    // you can unwrap/use an option like this
    match maybe_marrys_occupation {
        Some(occupation) => println!("occupation: {}", occupation),
        None => println!("No occupation found! => do nothing"),
    };

    // IMPORTANT NEVER USE .unwrap() => it panics if the option is None and is UNSAFE!!!
    println!("\nIMPORTANT NEVER USE .unwrap() => it panics if the option is None!!!");

    // unwrap safely
    let marrys_occupation = maybe_marrys_occupation.unwrap_or("N/A");
    let asdjkhk_occupation = maybe_asdjkhk_occupation.unwrap_or("N/A");
    println!("extracted marry's occupation is  : {:?}", marrys_occupation);
    println!("extracted asdjkhk's occupation is: {:?}", asdjkhk_occupation);

    // KEY LEARNING! 
    unwrap_option_with_questionmark_within_function_that_returns_option_or_result();
    
}

fn unwrap_option_with_questionmark_within_function_that_returns_option_or_result() -> Option<i32> {
    println!("\n\nYou can unwrap an option with `?` only within a function that returns an option or result");
    let res_option = try_division(19, 2);
    let res = res_option?;
    println!("res_option: {:?}", res_option);
    println!("res: {}", res);
    Option::Some(res)
}

// OPTION IS AN ENUM WITH
// Option::Some() // includes payload
// Option::None   // no payload
fn try_division(dividend: i32, divisor: i32) -> Option<i32> {
    match divisor {
        0 => None,
        _ => Some(dividend / divisor),
    }
}

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "arthur" => Option::Some("holochain architect"), // shortcuts to Some("..")
        "marry" => Option::Some("executive director"),   // shortcuts to Some("..")
        "nicolas" => Option::Some("core developer"),     // shortcuts to Some("..")
        _ => Option::None,                               // shortcuts to None
    }
}
