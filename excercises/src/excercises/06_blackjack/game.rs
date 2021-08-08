use rand::Rng;
use std::io;
use std::io::Write;
use std::{thread, time};

// 1,5 seconds wait time for dealer
const SLEEP_TIME: u64 = 1000;

// use termion::color;
// use termion::style;
// let str = format!("{}Red{}Green", color::Fg(color::Red), color::Fg(color::White));
// let str2 = format!("{}Red{}{}Bold", color::Fg(color::Red), color::Fg(color::White), style::Bold);

pub fn run() {
    // Bank initial cards
    let mut dealer_cards: Vec<Card> = vec![];
    dealer_cards.push(pick_card());
    dealer_cards.push(pick_card());
    let mut dealer_score = compute_score(&dealer_cards);
    print_title();
    println!("Dealer Score: ?");
    print_cards_hidden(&dealer_cards);

    // Player cards
    let mut player_cards: Vec<Card> = vec![];
    player_cards.push(pick_card());
    player_cards.push(pick_card());
    let mut player_score = compute_score(&player_cards);
    println!("Player Score: {}", player_score);
    print_cards(&player_cards);

    // while
    loop {
        // prompt user action
        println!("Your score is {}. What do you like to do?\n- draw another card (d)\n- stop (s)", player_score);
        print!("> ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice = choice.trim();
        if choice == "d" {
            player_cards.push(pick_card());
            print_title();

            println!("Dealer Score: ?");
            print_cards_hidden(&dealer_cards);
            player_score = compute_score(&player_cards);
            println!("Player Score: {}", player_score);
            print_cards(&player_cards);
            if player_score > 21 {
                print_title();
                println!("Dealer Score: {}", dealer_score);
                print_cards(&dealer_cards);
                println!("Player Score: {}", player_score);
                print_cards(&player_cards);
                print_game_end(dealer_score, player_score, "❌ BUSTED ❌".to_string());
                break;
            }

        } else if choice == "s" {
            // check if dealer has won
            if dealer_score > player_score {
                print_title();
                println!("Dealer Score: {}", dealer_score);
                print_cards(&dealer_cards);
                println!("Player Score: {}", player_score);
                print_cards(&player_cards);
                print_game_end(dealer_score, player_score, "❌ YOU LOST ❌".to_string());
                break;
            } else if dealer_score == player_score {
                print_title();
                println!("Dealer Score: {}", dealer_score);
                print_cards(&dealer_cards);
                println!("Player Score: {}", player_score);
                print_cards(&player_cards);
                print_game_end(dealer_score, player_score, "✋ It's a DRAW ✋".to_string());
                break;
            } else {
                print_title();
                println!("Dealer Score: {}", dealer_score);
                print_cards(&dealer_cards);
                println!("Player Score: {}", player_score);
                print_cards(&player_cards);
                println!("👀 Dealer revealer his hidden card...\n...\n...");
                while dealer_score < player_score {
                    // wait 1 sec
                    sleep();
                    // take another card and display
                    dealer_cards.push(pick_card());
                    dealer_score = compute_score(&dealer_cards);
                    // check endgame conditions
                    if dealer_score > 21 {
                        print_title();
                        println!("Dealer Score: {}", dealer_score);
                        print_cards(&dealer_cards);
                        println!("Player Score: {}", player_score);
                        print_cards(&player_cards);
                        print_game_end(dealer_score, player_score, "🎉 YOU WON 🎉".to_string());
                        break;
                    } else if dealer_score == player_score {
                        print_title();
                        println!("Dealer Score: {}", dealer_score);
                        print_cards(&dealer_cards);
                        println!("Player Score: {}", player_score);
                        print_cards(&player_cards);
                        print_game_end(dealer_score, player_score, "✋ It's a DRAW ✋".to_string());
                        break;
                    } else if dealer_score > player_score {
                        print_title();
                        println!("Dealer Score: {}", dealer_score);
                        print_cards(&dealer_cards);
                        println!("Player Score: {}", player_score);
                        print_cards(&player_cards);
                        print_game_end(dealer_score, player_score, "❌ YOU LOST ❌".to_string());
                        break;
                    }
                    print_title();
                    println!("Dealer Score: {}", dealer_score);
                    print_cards(&dealer_cards);
                    println!("Player Score: {}", player_score);
                    print_cards(&player_cards);
                    println!("🃏 Dealer has grabbed another card... \n...\n...");
                }
                break;
            }
        } else {
            println!("(INVALID INPUT)");
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, Clone)]
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
    let mut contains_ace: bool = false;
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
            Value::Ace => {
                contains_ace = true;
                11
            },
        };
        score += card_score;
    }
    if score > 21 && contains_ace {
        score -= 10
    }
    score
}

fn print_cards(cards: &Vec<Card>) {
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
fn print_cards_hidden(cards: &Vec<Card>) {
    let cards_draft = "┌─────┐┌─────┐\n\
                            │░░░░░|│v    |\n\
                            │░░░░░|│  c  |\n\
                            │░░░░░|│    v|\n\
                            └─────┘└─────┘\n";
    let mut cards_string: String = String::from(cards_draft);
    // inject values
    if cards[0].value == Value::Ten {
        cards_string = cards_string.replace("v ", cards[0].value_char());
        cards_string = cards_string.replace(" v", cards[0].value_char());
    } else {
        cards_string = cards_string.replace("v", cards[0].value_char());
        cards_string = cards_string.replace("v", cards[0].value_char());
    }
    // inject color
    cards_string = cards_string.replace("c", cards[0].color_char());
    // return
    println!("{}", cards_string);
}

fn sleep() {
    thread::sleep(time::Duration::from_millis(SLEEP_TIME));
}
fn print_title() {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("=== BLACKJACK ===");
}
// fn print_game_board(player_cards: Vec<Card>, dealer_cards: Vec<Card> )
fn print_game_end(dealer_score: u8, player_score: u8, status: String) {
    println!("{}\n- Your score is {}\n- The Dealer's score is {}", status, player_score, dealer_score);
    print!("> press any key to exit");
    io::stdout().flush().unwrap();
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    println!("...exiting");
}
