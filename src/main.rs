use std::{env, fs, path::Path};

use day1::{puzzle1::solve_day1_puzzle1, puzzle2::solve_day1_puzzle2};

pub mod day1;

fn read_file(file_name: &str) -> String {
    let input_path = env::var("INPUT_PATH").expect("INPUT_PATH environment variable must be set");
    let input_path = Path::new(&input_path);
    let full_path = input_path.join(file_name);

    fs::read_to_string(full_path).expect("unable to read day 1 puzzle 2 file")
}

fn main() {
    dotenvy::dotenv().expect("should be able to load .env file!");

    let day1_puzzle1_result = solve_day1_puzzle1();
    println!("day1 puzzle1: {day1_puzzle1_result}");

    let day1_puzzle2_result = solve_day1_puzzle2();
    println!("day1 puzzle2: {day1_puzzle2_result}");
}
