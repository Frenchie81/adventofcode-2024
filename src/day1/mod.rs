use std::fs;

pub fn puzzle1() -> i64 {
    let contents = read_file();
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();
    for line in contents.lines() {
        let splits: Vec<&str> = line.split("   ").collect();
        if splits.len() != 2 {
            continue;
        }

        let value1: i64 = splits[0].parse().expect("value1 should be int");
        let value2: i64 = splits[1].parse().expect("value2 should be int");

        left.push(value1);
        right.push(value2);
    }
    get_puzzle1_result(left, right)
}

fn read_file() -> String {
    fs::read_to_string("/home/bfrench/projects/adventofcode-2024/inputs/day1_1.txt")
        .expect("unable to read day 1 puzzle 1 file")
}

fn get_puzzle1_result(mut left: Vec<i64>, mut right: Vec<i64>) -> i64 {
    left.sort();
    right.sort();

    let mut right_index = 0;
    let right_len = right.len();
    let mut left_index = 0;
    let left_len = left.len();
    let mut distance: i64 = 0;

    println!("{left_len}-{right_len}");

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
    fn puzzle1_demo() {
        let left: Vec<i64> = vec![3, 4, 2, 1, 3, 3];
        let right: Vec<i64> = vec![4, 3, 5, 3, 9, 3];

        let result = get_puzzle1_result(left, right);

        assert_eq!(11, result);
    }
}
