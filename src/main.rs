use std::{env, fs, path::Path};

use day1::{puzzle1::solve_day1_puzzle1, puzzle2::solve_day1_puzzle2};
use day2::{puzzle1::solve_day2_puzzle1, puzzle2::solve_day2_puzzle2};
use day3::{puzzle1::solve_day3_puzzle1, puzzle2::solve_day3_puzzle2};
use day4::{puzzle1::solve_day4_puzzle1, puzzle2::solve_day4_puzzle2};
use day5::{puzzle1::solve_day5_puzzle1, puzzle2::solve_day5_puzzle2};
use day6::{puzzle1::solve_day6_puzzle1, puzzle2::solve_day6_puzzle2};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

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

    let day2_puzzle1_result = solve_day2_puzzle1();
    println!("day2 puzzle1: {day2_puzzle1_result}");

    let day2_puzzle2_result = solve_day2_puzzle2();
    println!("day2 puzzle1: {day2_puzzle2_result}");

    let day3_puzzle1_result = solve_day3_puzzle1();
    println!("day3 puzzle1: {day3_puzzle1_result}");

    let day3_puzzle2_result = solve_day3_puzzle2();
    println!("day3 puzzle2: {day3_puzzle2_result}");

    let day4_puzzle1_result = solve_day4_puzzle1();
    println!("day4 puzzle1: {day4_puzzle1_result}");

    let day4_puzzle2_result = solve_day4_puzzle2();
    println!("day4 puzzle2: {day4_puzzle2_result}");

    let day5_puzzle1_result = solve_day5_puzzle1();
    println!("day5 puzzle1: {day5_puzzle1_result}");

    let day5_puzzle2_result = solve_day5_puzzle2();
    println!("day5 puzzle2: {day5_puzzle2_result}");

    let day6_puzzle1_result = solve_day6_puzzle1();
    println!("day6 puzzle1: {day6_puzzle1_result}");

    let day6_puzzle2_result = solve_day6_puzzle2();
    println!("day6 puzzle2: {day6_puzzle2_result}");
}
