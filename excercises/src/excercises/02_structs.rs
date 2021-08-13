use chrono::NaiveDate; // for easy date computation

pub fn run() {
    println!("=== STRUCTS ===");
    println!("=== (A) PERSON Struct ===");
    println!("generate person:");
    let mut p: Person = Person::new("frakie","miller", 30);
    p.print();
    p.happy_birthday();
    p.happy_birthday();
    p.print();
    p.set_last_name("Rogan");
    p.print();
    println!("p.to_string() : {}", p.to_string());

    println!("\n\n=== (B) Date ===");
    let d = Date::new("2021-08-02");
    println!("formatted_date_str: {}", d.get_formatted_date_str()); // => "2. August 2021"
    println!("formatted_date_str: {}", d.get_date_str()); // => "2021-08-02"
    println!("unix              : {}", d.get_unix()); // => 1627862400
}

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}
impl Person {
    fn new(first_name: &str, last_name: &str, age: u8) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age: age,
        }
    }
    fn print(&self) {
        println!("{} {} is {} years old.", self.first_name, self.last_name, self.age);
    }
    fn to_string(&self) -> String {
        format!("{} {} is {} years old.", self.first_name, self.last_name, self.age)
    }
    fn happy_birthday(&mut self) {
        self.age += 1;
    }
    fn set_last_name(&mut self, new_last_name: &str) {
        self.last_name = new_last_name.to_string();
    }
}

#[derive(Debug)]
// At 03:14:08 UTC on Tuesday, 19 January 2038,
// 32-bit versions of the Unix timestamp will
// cease to work, as it will overflow the largest
// value that can be held in a signed 32-bit number
// => the library we use (chromo) returns an i64
//    as unix timestamp, so we have some more time
struct Date {
    date_str: String,
    unix_timestamp: i64,
}
impl Date {
    fn new(date_str: &str) -> Date {
        let (year, month, day) = Date::date_str_to_ymd(date_str);
        let d = NaiveDate::from_ymd(year, month, day);
        Date {
            date_str: date_str.to_string(),
            unix_timestamp: d.and_hms(0, 0, 0).timestamp(),
        }
    }
    fn get_unix(&self) -> i64 {
        self.unix_timestamp
    }
    fn get_date_str(&self) -> String {
        self.date_str.clone()
    }
    fn get_formatted_date_str(&self) -> String {
        let (year, month, day) = Date::date_str_to_ymd(&self.date_str);
        let month_name: String = match month {
            1 => "January".to_string(),
            2 => "February".to_string(),
            3 => "March".to_string(),
            4 => "April".to_string(),
            5 => "May".to_string(),
            6 => "June".to_string(),
            7 => "July".to_string(),
            8 => "August".to_string(),
            9 => "September".to_string(),
            10 => "October".to_string(),
            11 => "November".to_string(),
            12 => "December".to_string(),
            _ => panic!("Invalid date_str entered. Got: {}", self.date_str),
        };
        // return formatted string
        format!("{}. {} {}", day, month_name, year)
    }
    // helper function to transform datestr to numeric values
    // => 2021-12-24 => (2021, 12, 24)
    fn date_str_to_ymd(date_str: &str) -> (i32, u32, u32) {
        let date_split: Vec<&str> = date_str.split("-").collect();
        let year: i32 = date_split[0].parse::<i32>().unwrap();
        let month: u32 = date_split[1].parse::<u32>().unwrap();
        let day: u32 = date_split[2].parse::<u32>().unwrap();
        (year, month, day)
    }
}
