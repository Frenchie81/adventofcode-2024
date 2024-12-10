use std::{env, fs, path::Path};

use day10::puzzle2::solve_day10_puzzle2;

pub mod day1;
pub mod day10;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

fn read_file(file_name: &str) -> String {
    let input_path = env::var("INPUT_PATH").expect("INPUT_PATH environment variable must be set");
    let input_path = Path::new(&input_path);
    let full_path = input_path.join(file_name);

    fs::read_to_string(full_path).expect("unable to read day 1 puzzle 2 file")
}

fn main() {
    dotenvy::dotenv().expect("should be able to load .env file!");

    let result = solve_day10_puzzle2();
    println!("{result}");
}
