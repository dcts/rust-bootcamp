use rand::Rng;
use std::{thread, time};

const FILLED: char = '█';
const EMPTY: char = '░';

const MIN: u8 = 0;
const MAX: u8 = 30;

const SLEEP_TIME: u64 = 100;

pub fn run(size: u8) {
    println!("=== BUBBLE SORT ===");
    println!("size of vector to sort: {}", size);

    let mut target = generate_random_vector(size);
    for x in 1..=100 {
        println!("\n\n\n\n\n\n\n\n\n\n\nITERATION:{}", x);
        display_vector(&target);
        target = generate_random_vector(size);
        sleep();
    }
}

fn generate_random_vector(size: u8) -> Vec<u8> {
    // initialize vector
    let mut random_vector: Vec<u8> = vec![];
    while random_vector.len() < (size as usize) {
        let random_value: u8 = rand::thread_rng().gen_range(MIN..=MAX);
        random_vector.push(random_value);
    }
    random_vector
}

fn display_vector(vector: &Vec<u8>) {
    for value in vector {
        display_line(value.clone());
    }
}
fn display_line(num: u8) {
    // create string to append
    let mut line = String::from("");
    for x in 0..MAX {
        if x < num {
            line.push(FILLED);
        } else {
            line.push(EMPTY);
        }
    }
    println!("{}", line);
}

fn sleep() {
    thread::sleep(time::Duration::from_millis(SLEEP_TIME));
}
