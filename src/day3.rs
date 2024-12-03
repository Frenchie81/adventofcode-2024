use regex::Regex;

pub mod puzzle1;
pub mod puzzle2;

fn get_mul_result(mul: &str) -> i64 {
    let nums_re = Regex::new(r"\d*,\d*").expect("the regex should parse");
    let nums = nums_re
        .find(mul)
        .expect("comma separated values should always be present");
    let splits: Vec<&str> = nums.as_str().split(',').collect();
    let val1: i64 = splits[0].parse().expect("value1 should always be a number");
    let val2: i64 = splits[1].parse().expect("value2 should always be a number");
    val1 * val2
}
