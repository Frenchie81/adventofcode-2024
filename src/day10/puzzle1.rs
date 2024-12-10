use crate::read_file;

use super::Map;

pub fn solve_day10_puzzle1() -> i64 {
    get_result(read_file("day10.txt").as_str())
}

fn get_result(content: &str) -> i64 {
    let map = Map::parse(content);
    map.get_map_total()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let content = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        let result = get_result(content);

        assert_eq!(36, result);
    }

    #[test]
    fn file() {
        dotenvy::dotenv().expect("should be able to load .env file!");

        let result = solve_day10_puzzle1();

        assert_eq!(816, result);
    }
}
