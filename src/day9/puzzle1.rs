use crate::{
    day9::{get_checksum, move_files},
    read_file,
};

use super::parse_content;

pub fn solve_day9_puzzle1() -> i64 {
    get_result(read_file("day9.txt").as_str())
}

fn get_result(content: &str) -> i64 {
    let mut parsed = parse_content(content);
    move_files(&mut parsed);
    get_checksum(&parsed)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn demo() {
        let content = "2333133121414131402";

        let result = get_result(content);

        assert_eq!(1928, result);
    }

    #[test]
    fn file() {
        dotenvy::dotenv().expect("should be able to load .env file!");

        let result = solve_day9_puzzle1();

        assert_eq!(6463499258318, result);
    }
}
