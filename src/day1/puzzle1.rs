use crate::read_file;

use super::split_contents;

pub fn solve_day1_puzzle1() -> i64 {
    let contents = read_file("day1.txt");
    let (left, right) = split_contents(contents);

    get_puzzle1_result(left, right)
}

fn get_puzzle1_result(mut left: Vec<i64>, mut right: Vec<i64>) -> i64 {
    left.sort();
    right.sort();

    let mut right_index = 0;
    let right_len = right.len();
    let mut left_index = 0;
    let left_len = left.len();
    let mut distance: i64 = 0;

    while left_index < left_len {
        let left_value = left[left_index];
        if right_index < right_len {
            let right_value = right[right_index];
            let diff = left_value - right_value;
            distance += diff.abs();
            right_index += 1;
        } else {
            break;
        }
        left_index += 1;
    }
    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let left: Vec<i64> = vec![3, 4, 2, 1, 3, 3];
        let right: Vec<i64> = vec![4, 3, 5, 3, 9, 3];

        let result = get_puzzle1_result(left, right);

        assert_eq!(11, result);
    }

    #[test]
    fn file() {
        dotenvy::dotenv().expect("should be able to load .env file!");

        let result = solve_day1_puzzle1();

        assert_eq!(1341714, result);
    }
}
