
pub fn run() {
    println!("=== OPTIONS ===");

    // get an option like this
    let marrys_occupation = match get_occupation("marry") {
        Some(occupation) => occupation,
        None => "No occupation found!",
    };
    println!("marry's occupation is: {}", marrys_occupation);

    let res_option = divide(19, 2)?;
    println!("res = {:?}", res_option);
    // let res = res_option?;
    // println!("res = {:?}", res);
}

// OPTION IS AN ENUM WITH
// Option::Some() // includes payload
// Option::None   // no payload
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None
    }
    return Some(a/b);
}

// RESULT IS AN ENUM WITH
// Result::Ok()
// Result::Err()
// fn better_divide<T>(a: T, b: T) -> Result<T> {
//     if a / b == 0 {
//         return Err("Error!".to_string());
//     }
//     return Ok(a/b);
// }

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "arthur" => Option::Some("holochain architect"), // shortcuts to Some("..")
        "marry" => Option::Some("executive director"),   // shortcuts to Some("..")
        "nicolas" => Option::Some("core developer"),     // shortcuts to Some("..")
        _ => Option::None,                               // shortcuts to None
    }
}
