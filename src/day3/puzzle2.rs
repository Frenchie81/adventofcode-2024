use regex::Regex;

use crate::read_file;

use super::get_mul_result;

pub fn solve_day3_puzzle2() -> i64 {
    let contents = read_file("day3.txt");

    get_result(contents.as_str())
}

fn get_result(contents: &str) -> i64 {
    let filtered = filter_do(contents);
    let mut sum = 0_i64;
    let mul_re = Regex::new(r"mul\(\d*,\d*\)").expect("the regex should parse");

    let muls: Vec<&str> = mul_re
        .find_iter(filtered.as_str())
        .map(|m| m.as_str())
        .collect();

    for mul in muls {
        sum += get_mul_result(mul);
    }

    sum
}

fn filter_do(content: &str) -> String {
    let mut new_content = String::new();
    let mut capture = true;

    for (i, c) in content.char_indices() {
        if capture {
            new_content.push(c);
            let end = i + 7;
            let dont_test = content.get(i..end).unwrap_or("");

            if dont_test == r"don't()" {
                capture = false;
            }
        } else {
            let end = i + 4;
            let do_test = content.get(i..end).unwrap_or("");
            if do_test == r"do()" {
                capture = true;
            }
        }
    }

    new_content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let contents = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let result = get_result(contents);

        assert_eq!(48, result);
    }

    #[test]
    fn file() {
        dotenvy::dotenv().expect("should be able to load .env file!");

        let result = solve_day3_puzzle2();

        assert_eq!(89823704, result);
    }
}
