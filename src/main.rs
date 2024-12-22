use std::{env, fs, path::Path};

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
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

    let result = day19::solve_puzzle_1();
    println!("result: {result:?}");
}
