use crate::read_file;

use super::split_contents;

pub fn solve_day1_puzzle2() -> i64 {
    let contents = read_file("day1.txt");
    let (left, right) = split_contents(contents);
    get_puzzle2_result(left, right)
}

fn get_puzzle2_result(left: Vec<i64>, right: Vec<i64>) -> i64 {
    let mut similarity = 0_i64;

    for left_value in left {
        let mut found_count = 0_i64;
        for right_value in right.iter() {
            if left_value == *right_value {
                found_count += 1;
            }
        }

        if found_count > 0 {
            similarity += left_value * found_count;
        }
    }

    similarity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle2_demo() {
        let left: Vec<i64> = vec![3, 4, 2, 1, 3, 3];
        let right: Vec<i64> = vec![4, 3, 5, 3, 9, 3];

        let result = get_puzzle2_result(left, right);

        assert_eq!(31, result);
    }
}
