use crate::read_file;

use super::*;

pub fn solve_day11_puzzle1() -> i64 {
    get_result(read_file("day11.txt").as_str())
}

fn get_result(content: &str) -> i64 {
    let stones = parse_content(content);
    run(stones, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let content = "125 17";

        let result = get_result(content);

        assert_eq!(55312, result);
    }

    #[test]
    fn file() {
        dotenvy::dotenv().expect("should be able to load .env file!");

        let result = solve_day11_puzzle1();

        assert_eq!(220999, result);
    }
}
