#[path = "excercises/01_functions.rs"] mod functions;
#[path = "excercises/02_structs.rs"] mod structs;
#[path = "excercises/03_enums.rs"] mod enums;
#[path = "excercises/04_collatz.rs"] mod collatz;
#[path = "excercises/05_bubble_sort.rs"] mod bubble_sort;
#[path = "excercises/06_blackjack.rs"] mod blackjack;
#[path = "excercises/07_game_of_life.rs"] mod game_of_life;
use std::env;

fn main() {
    // take first argument as command
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    if command == "functions" {
        functions::run();
    } else if command == "structs" {
        structs::run();
    } else if command == "enums" {
        enums::run();
    } else if command == "collatz" {
        collatz::run();
    } else if command == "bubble" {
        bubble_sort::run(20);
    } else if command == "blackjack" {
        blackjack::run();
    } else if command == "game_of_life" || command == "gol" {
        game_of_life::run();
    }
}
