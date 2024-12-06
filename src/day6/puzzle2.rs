use crate::read_file;

use super::Map;

pub fn solve_day6_puzzle2() -> i64 {
    let contents = read_file("day6.txt");
    get_result(contents.as_str())
}

fn get_result(content: &str) -> i64 {
    let mut count = 0;
    let chars: Vec<char> = content.chars().collect();
    let char_count = chars.len();
    for x in 0..char_count {
        if chars[x] == '#' || chars[x] == '^' || chars[x] == '\n' {
            continue;
        }
        let mut new_content = String::new();
        for (y, c) in chars.iter().enumerate() {
            if y == x {
                new_content.push('#');
            } else {
                new_content.push(*c);
            }
        }

        let max_iterations = 10000;
        let mut current_iteration = 0;
        let mut map = Map::parse(new_content.as_str());

        while map.move_guard() {
            if current_iteration == max_iterations {
                break;
            }

            current_iteration += 1;
        }

        if current_iteration == max_iterations {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let content = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let result = get_result(content);

        assert_eq!(6, result);
    }
}
