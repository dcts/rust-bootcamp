use rand::Rng;

pub fn run() {
    println!("=== BLACKJACK ===");
    let random_card: Card = Card::new(pick_value(), pick_color());
    println!("{:?}", random_card);
}

#[derive(Debug)]
struct Card {
    value: Value,
    color: Color,
}
impl Card {
    // constructor
    fn new(value: Value, color: Color) -> Card {
        Card {
            value: value,
            color: color,
        }
    }
    // return char
}

#[derive(Debug)]
enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
fn pick_value() -> Value {
    match rand::thread_rng().gen_range(2..=14) {
        2 => Value::Two,
        3 => Value::Three,
        4 => Value::Four,
        5 => Value::Five,
        6 => Value::Six,
        7 => Value::Seven,
        8 => Value::Eight,
        9 => Value::Nine,
        10 => Value::Ten,
        11 => Value::Jack,
        12 => Value::Queen,
        13 => Value::King,
        14 => Value::Ace,
        _ => panic!("Randomly generated value is out of bound. Allowed 2-14."),
    }
}

#[derive(Debug)]
enum Color {
    Heart,
    Diamond,
    Spade,
    Club,
}
fn pick_color() -> Color {
    match rand::thread_rng().gen_range(0..=3) {
        0 => Color::Heart,
        1 => Color::Diamond,
        2 => Color::Spade,
        3 => Color::Club,
        _ => panic!("Randomly generated value is out of bound. Allowed 0-3."),
    }
}
