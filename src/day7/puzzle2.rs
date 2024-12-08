use crate::read_file;

use super::parse_contents;

pub fn solve_day7_puzzle2() -> i64 {
    get_result(read_file("day7.txt").as_str())
}

fn get_result(content: &str) -> i64 {
    let equations = parse_contents(content);

    equations
        .iter()
        .filter(|x| x.can_solve(true))
        .map(|x| x.test_value)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let content = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let result = get_result(content);

        assert_eq!(11387, result);
    }
}
