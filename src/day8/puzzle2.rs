use crate::read_file;

use super::Map;

pub fn solve_day8_puzzle2() -> i64 {
    get_result(read_file("day8.txt").as_str())
}

fn get_result(content: &str) -> i64 {
    let mut map = Map::parse(content);
    map.apply_frequencies_with_resonance();
    map.render();
    map.count_tiles_with_antinodes()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn demo() {
        let content = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let result = get_result(content);

        assert_eq!(34, result);
    }

    #[test]
    fn file() {
        dotenvy::dotenv().expect("should be able to load .env file!");

        let result = solve_day8_puzzle2();

        assert_eq!(1352, result);
    }
}
