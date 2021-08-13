use chrono::NaiveDate; // for easy date computation

pub fn run() {
    println!("=== ENUMS ===");
    println!("=== (A) PERSON ===");
    // println!("generate person:");
    let person_1: Person = Person::new("Frakie","Miller", 30, Gender::Male);
    let person_2: Person = Person::new("Alice","Wonderland", 30, Gender::Female);
    person_1.greet();
    person_2.greet();
}

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    gender: Gender,
}
impl Person {
    fn new(first_name: &str, last_name: &str, age: u8, gender: Gender) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age: age,
            gender: gender,
        }
    }
    // fn print(&self) {
    //     println!("{} {} is {} years old.", self.first_name, self.last_name, self.age);
    // }
    // fn to_string(&self) -> String {
    //     format!("{} {} is {} years old.", self.first_name, self.last_name, self.age)
    // }
    // fn happy_birthday(&mut self) {
    //     self.age += 1;
    // }
    // fn set_last_name(&mut self, new_last_name: &str) {
    //     self.last_name = new_last_name.to_string();
    // }
    fn greet(&self) {
        match self.gender {
            Gender::Male => println!("Hello Mr. {}", self.last_name),
            Gender::Female => println!("Hello Mrs. {}", self.last_name),
        }
    }
}
#[derive(Debug)]
enum Gender {
    Male,
    Female,
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

