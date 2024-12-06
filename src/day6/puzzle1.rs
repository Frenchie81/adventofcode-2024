use crate::read_file;

use super::Map;

pub fn solve_day6_puzzle1() -> i64 {
    let content = read_file("day6.txt");
    get_result(content.as_str())
}

fn get_result(content: &str) -> i64 {
    let mut map = Map::parse(content);
    while map.move_guard() {}

    let mut visited_count: i64 = 0;
    for tile_line in map.tiles {
        for tile in tile_line {
            if tile.visited {
                visited_count += 1;
            }
        }
    }

    visited_count
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

        assert_eq!(41, result);
    }
}
