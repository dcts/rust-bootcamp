use std::env;

mod example_01_print;
mod example_02_vars;
mod example_03_types;
mod example_04_strings;
mod example_05_tuples;
mod example_06_arrays;
mod example_07_vectors;
mod example_08_conditionals;
mod example_09_loops;
mod example_10_functions;
mod example_11_ref_pointers;
mod example_12_structs;
mod example_13_enums;
mod example_14_cli;
mod example_15_options;
mod example_16_hashmap;
mod example_17_traits;
mod example_18_matching;

fn main() {
    let args: Vec<String> = env::args().collect();
    let run_argument = args.get(1);

    match run_argument {
        Some(string) => {
            match string.as_str() {
                "print" => example_01_print::run(),
                "vars" => example_02_vars::run(),
                "types" => example_03_types::run(),
                "strings" => example_04_strings::run(),
                "tuples" => example_05_tuples::run(),
                "arrays" => example_06_arrays::run(),
                "vectors" => example_07_vectors::run(),
                "conditionals" => example_08_conditionals::run(),
                "loops" => example_09_loops::run(),
                "functions" => example_10_functions::run(),
                "ref_pointers" => example_11_ref_pointers::run(),
                "structs" => example_12_structs::run(),
                "enums" => example_13_enums::run(),
                "cli" => example_14_cli::run(),
                "options" => example_15_options::run(),
                "hashmap" => example_16_hashmap::run(),
                "traits" => example_17_traits::run(),
                "matching" => example_18_matching::run(),
                _ => println!("❌ invalid argument: {} ", string.as_str()),
            }
        },
        None => println!("❌ argument missing. Please run `cargo run ARGUMENT`\n  e.g. cargo run loops\n  e.g. cargo run functions"),
    }
}

