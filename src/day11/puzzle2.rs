use super::*;
use crate::read_file;

pub fn solve_day11_puzzle2() -> i64 {
    get_result(read_file("day11.txt").as_str())
}

fn get_result(content: &str) -> i64 {
    let stones = parse_content(content);
    run(stones, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file() {
        dotenvy::dotenv().expect("should be able to load .env file!");

        let result = solve_day11_puzzle2();

        assert_eq!(261936432123724, result);
    }
}
