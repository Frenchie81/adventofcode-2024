use regex::Regex;

use crate::read_file;

use super::get_mul_result;

pub fn solve_day3_puzzle1() -> i64 {
    let contents = read_file("day3.txt");

    get_result(contents.as_str())
}

fn get_result(contents: &str) -> i64 {
    let mut sum = 0_i64;
    let mul_re = Regex::new(r"mul\(\d*,\d*\)").expect("the regex should parse");

    let muls: Vec<&str> = mul_re.find_iter(contents).map(|m| m.as_str()).collect();

    for mul in muls {
        sum += get_mul_result(mul);
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
