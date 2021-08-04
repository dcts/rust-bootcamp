use rand::Rng;

// use termion::color;
// use termion::style;
// let str = format!("{}Red{}Green", color::Fg(color::Red), color::Fg(color::White));
// let str2 = format!("{}Red{}{}Bold", color::Fg(color::Red), color::Fg(color::White), style::Bold);

pub fn run() {
    println!("=== BLACKJACK ===");
    // Bank initial cards
    let mut dealer_cards: Vec<Card> = vec![];
    dealer_cards.push(pick_card());
    dealer_cards.push(pick_card());
    let dealer_score = compute_score(&dealer_cards);
    println!("Dealer Score: {}", dealer_score);
    print_cards(dealer_cards);

    // Player cards
    let mut player_cards: Vec<Card> = vec![];
    player_cards.push(pick_card());
    player_cards.push(pick_card());
    let player_score = compute_score(&player_cards);
    println!("Player Score: {}", player_score);
    print_cards(player_cards);
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
    fn to_string(&self) -> String {
        let card_draft = "┌─────┐\n\
                               │v    |\n\
                               │  c  |\n\
                               │    v|\n\
                               └─────┘";
        let mut card_string: String = String::from(card_draft);
        // inject values
        if self.value == Value::Ten {
            card_string = card_string.replace("v ", self.value_char());
            card_string = card_string.replace(" v", self.value_char());
        } else {
            card_string = card_string.replace("v", self.value_char());
            card_string = card_string.replace("v", self.value_char());
        }
        // inject color
        card_string = card_string.replace("c", self.color_char());
        // return
        card_string
    }

    // return value char
    fn value_char(&self) -> &str {
        match self.value {
            Value::Two => "2",
            Value::Three => "3",
            Value::Four => "4",
            Value::Five => "5",
            Value::Six => "6",
            Value::Seven => "7",
            Value::Eight => "8",
            Value::Nine => "9",
            Value::Ten => "10",
            Value::Jack => "J",
            Value::Queen => "Q",
            Value::King => "K",
            Value::Ace => "A",
        }
    }
    // return color char
    fn color_char(&self) -> &str {
        match self.color {
            Color::Heart => "♥",
            Color::Diamond => "♦",
            Color::Spade => "♠",
            Color::Club => "♣",
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
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

fn pick_card() -> Card {
    Card::new(pick_value(), pick_color())
}
fn compute_score(cards: &Vec<Card>) -> u8 {
    let mut score = 0;
    for card in cards.iter() {
        let card_score: u8 = match card.value {
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 10,
            Value::Queen => 10,
            Value::King => 10,
            Value::Ace => 11,
        };
        score += card_score;
    }
    score
}
fn print_cards(cards: Vec<Card>) {
    // generate card splits
    let mut card_splits: Vec<Vec<String>> = vec![];
    for card in cards {
        let card_string = String::from(card.to_string());
        let vec_temporary: Vec<&str> = card_string.split("\n").collect();
        // CMON THIS IS INSANE...
        // THERE HAS TO BE A BETTER
        // OF DOING THIS SHIT....
        let mut vec: Vec<String> = vec![];
        for temp in vec_temporary {
            vec.push(temp.to_owned());
        }
        card_splits.push(vec);
    }

    // iterate over each line
    let mut final_string = String::from("");
    for line in 1..=5 {
        for split in card_splits.iter() {
            final_string.push_str(&split[line-1]);
        }
        final_string.push_str("\n");
    }
    println!("{}",final_string);
}
