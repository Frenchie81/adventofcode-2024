use regex::Regex;

use crate::read_file;

pub fn solve_day3_puzzle1() -> i64 {
    let contents = read_file("day3.txt");

    get_result(contents.as_str())
}

fn get_result(contents: &str) -> i64 {
    let mut sum = 0_i64;
    let mul_re = Regex::new(r"mul\(\d*,\d*\)").expect("the regex should parse");
    let nums_re = Regex::new(r"\d*,\d*").expect("the regex should parse");

    let muls: Vec<&str> = mul_re.find_iter(contents).map(|m| m.as_str()).collect();

    for mul in muls {
        let nums = nums_re
            .find(mul)
            .expect("comma separated values should always be present");
        let splits: Vec<&str> = nums.as_str().split(',').collect();
        let val1: i64 = splits[0].parse().expect("value1 should always be a number");
        let val2: i64 = splits[1].parse().expect("value2 should always be a number");
        sum += val1 * val2;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let contents = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let result = get_result(contents);

        assert_eq!(161, result);
    }
}
