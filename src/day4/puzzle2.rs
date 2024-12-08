use crate::read_file;

pub fn solve_day4_puzzle2() -> i64 {
    let contents = read_file("day4.txt");

    get_result(contents.as_str())
}

fn get_result(contents: &str) -> i64 {
    let mut count = 0_i64;

    let mut grid = Vec::<Vec<char>>::new();
    for line in contents.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let grid_len = grid.len();

    if grid_len < 3 {
        return count;
    }

    let mut x = 1;
    while x < grid_len - 1 {
        let mut y = 1;
        let row_len = grid[x].len();
        while y < row_len - 1 {
            let test = grid[x][y];
            if test == 'A' {
                let top_left = grid[x - 1][y - 1];
                let top_right = grid[x - 1][y + 1];
                let bottom_left = grid[x + 1][y - 1];
                let bottom_right = grid[x + 1][y + 1];

                let word1: String = [top_left, test, bottom_right].iter().collect();
                let word2: String = [top_right, test, bottom_left].iter().collect();

                if (word1 == "MAS" || word1 == "SAM") && (word2 == "MAS" || word2 == "SAM") {
                    count += 1;
                }
            }
            y += 1;
        }
        x += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let contents = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let result = get_result(contents);

        assert_eq!(9, result);
    }

    #[test]
    fn file() {
        dotenvy::dotenv().expect("should be able to load .env file!");

        let result = solve_day4_puzzle2();

        assert_eq!(1875, result);
    }
}
