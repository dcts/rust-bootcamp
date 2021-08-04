use rand::Rng;
use std::{thread, time};

const FILLED: char = '█';
const EMPTY: char = '░';

const MIN: u8 = 0;
const MAX: u8 = 30;

const SLEEP_TIME: u64 = 50000;

pub fn run(size: u8) {
    println!("=== BUBBLE SORT ===");
    println!("size of vector to sort: {}", size);

    let target = generate_random_vector(size);

    bubble_sort(target.clone());
}

fn bubble_sort(mut target: Vec<u8>) {
    let mut iteration_count = 0;
    let mut done: bool = false;
    while done == false {
        done = true;
        for indx in 0..target.len()-1-iteration_count {
            print_huge_gap();
            println!("ŘUNNING ITERATION {}_{}", iteration_count, indx);
            if target[indx] > target[indx+1] {
                let lower_value: u8 = target[indx+1];
                target[indx+1] = target[indx];
                target[indx] = lower_value;
                done = false;
            }
            display_vector(&target);
            println!("\n");
            sleep();
        }
        iteration_count += 1;
    }
    // FINISHED
    print_huge_gap();
    println!("🎉 FINISHED 🎉");
    display_vector(&target);
    println!("\n");
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

fn print_huge_gap() {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
}
