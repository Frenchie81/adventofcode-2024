use crate::read_file;
use std::cmp::Ordering;

use super::split_contents;

pub fn solve_day2_puzzle1() -> i64 {
    let contents = read_file("day2.txt");
    let lines = split_contents(contents);
    get_result(lines)
}

fn get_result(lines: Vec<Vec<i64>>) -> i64 {
    let mut count = 0_i64;
    for line in lines {
        if is_line_safe(line) {
            count += 1;
        }
    }
    count
}

fn is_line_safe(line: Vec<i64>) -> bool {
    let mut i = 1;
    let mut ordering: Option<Ordering> = None;

    while i < line.len() {
        let cur_val = line[i - 1];
        let next_val = line[i];

        let next_order = cur_val.cmp(&next_val);

        if next_order == Ordering::Equal {
            return false;
        }

        if let Some(current_order) = ordering {
            if current_order != next_order {
                return false;
            }
        } else {
            ordering = Some(next_order);
        }

        let diff = (cur_val - next_val).abs();
        if diff > 3 {
            return false;
        }

        i += 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let line1: Vec<i64> = vec![7, 6, 4, 2, 1];
        let line2: Vec<i64> = vec![1, 2, 7, 8, 9];
        let line3: Vec<i64> = vec![9, 7, 6, 2, 1];
        let line4: Vec<i64> = vec![1, 3, 2, 4, 5];
        let line5: Vec<i64> = vec![8, 6, 4, 4, 1];
        let line6: Vec<i64> = vec![1, 3, 6, 7, 9];
        let lines = vec![line1, line2, line3, line4, line5, line6];

        let result = get_result(lines);

        assert_eq!(2, result);
    }
}
