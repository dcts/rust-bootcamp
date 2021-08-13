use chrono::NaiveDate; // for easy date computation

pub fn run() {
    println!("=== ENUMS ===");
    println!("=== (A) PERSON ===");
    // println!("generate person:");
    let person_1: Person = Person::new("Frakie","Miller", 30, Gender::Male);
    let person_2: Person = Person::new("Alice","Wonderland", 30, Gender::Female);
    person_1.greet();
    person_2.greet();

    println!("\n=== (B) Date ===");
    let d = Date::new("2021-08-02");
    println!("formatted_date_str: {}", d.get_formatted_date_str()); // => "2. August 2021"
    println!("formatted_date_str: {}", d.get_date_str()); // => "2021-08-02"
    println!("unix              : {}", d.get_unix()); // => 1627862400
    println!("get_season        : {}", d.get_season()); // => Summer
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
    month: Month,
    season: Season,
}
impl Date {
    fn new(date_str: &str) -> Date {
        let (year, month, day) = Date::date_str_to_ymd(date_str);
        let d = NaiveDate::from_ymd(year, month, day);
        let (month_enum, season_enum): (Month, Season) = match month {
            1 => (Month::January, Season::Winter),
            2 => (Month::February, Season::Winter),
            3 => (Month::March, Season::Spring),
            4 => (Month::April, Season::Spring),
            5 => (Month::May, Season::Spring),
            6 => (Month::June, Season::Summer),
            7 => (Month::July, Season::Summer),
            8 => (Month::August, Season::Summer),
            9 => (Month::September, Season::Autumn),
            10 => (Month::October, Season::Autumn),
            11 => (Month::November, Season::Autumn),
            12 => (Month::December, Season::Winter),
            _ => panic!("Invalid date_str entered. Got: {}", date_str),
        };
        Date {
            date_str: date_str.to_string(),
            unix_timestamp: d.and_hms(0, 0, 0).timestamp(),
            month: month_enum,
            season: season_enum,
        }
    }
    fn get_unix(&self) -> i64 {
        self.unix_timestamp
    }
    fn get_date_str(&self) -> String {
        self.date_str.clone()
    }
    fn get_formatted_date_str(&self) -> String {
        let (year, _, day) = Date::date_str_to_ymd(&self.date_str);
        let month_name: String = format!("{:?}",self.month);
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
    fn get_season(&self) -> String {
        format!("{:?}", self.season)
    }
}
#[derive(Debug)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Debug)]
enum Season {
    Winter,
    Summer,
    Spring,
    Autumn,
}
