use crate::read_file;

pub fn solve_day8_puzzle1() -> i64 {
    get_result(read_file("day8.txt").as_str())
}

fn get_result(content: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::{get_result, solve_day8_puzzle1};

    #[test]
    fn demo() {
        let content = "";

        let result = get_result(content);

        assert_eq!(14, result);
    }

    #[test]
    fn file() {
        dotenvy::dotenv().expect("should be able to load .env file!");

        let result = solve_day8_puzzle1();

        panic!("not verified");
    }
}
