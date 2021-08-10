pub fn run() {
    // get an option like this
    let marrys_occupation = match get_occupation("marry") {
        Some(occupation) => occupation,
        None => "No occupation found!",
    };
    println!("marry's occupation is: {}", marrys_occupation);
}

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "arthur" => Some("holochain architect"),
        "marry" => Some("executive director"),
        "nicolas" => Some("core developer"),
        _ => None,
    }
}
