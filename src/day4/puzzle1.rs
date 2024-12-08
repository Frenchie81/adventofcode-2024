use crate::read_file;

pub fn solve_day4_puzzle1() -> i64 {
    let contents = read_file("day4.txt");

    get_result(contents.as_str())
}

fn get_result(contents: &str) -> i64 {
    let mut count = 0_i64;

    let mut grid = Vec::<Vec<char>>::new();
    for line in contents.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let mut x = 0;
    let grid_len = grid.len();

    while x < grid_len {
        let mut y = 0;
        let row_len = grid[x].len();
        while y < row_len {
            let temp = grid[x][y];
            if temp == 'X' {
                // right
                if y < row_len - 3 {
                    let word = [grid[x][y], grid[x][y + 1], grid[x][y + 2], grid[x][y + 3]];
                    let word: String = word.iter().collect();
                    if word == "XMAS" {
                        count += 1;
                    }
                }
                // left
                if y > 2 {
                    let word = [grid[x][y], grid[x][y - 1], grid[x][y - 2], grid[x][y - 3]];
                    let word: String = word.iter().collect();
                    if word == "XMAS" {
                        count += 1;
                    }
                }
                // up
                if x > 2 {
                    let word = [grid[x][y], grid[x - 1][y], grid[x - 2][y], grid[x - 3][y]];
                    let word: String = word.iter().collect();
                    if word == "XMAS" {
                        count += 1;
                    }
                }
                // down
                if (x + 3) < grid_len {
                    let word = [grid[x][y], grid[x + 1][y], grid[x + 2][y], grid[x + 3][y]];
                    let word: String = word.iter().collect();
                    if word == "XMAS" {
                        count += 1;
                    }
                }
                // left up
                if x > 2 && y > 2 {
                    let word = [
                        grid[x][y],
                        grid[x - 1][y - 1],
                        grid[x - 2][y - 2],
                        grid[x - 3][y - 3],
                    ];
                    let word: String = word.iter().collect();
                    if word == "XMAS" {
                        count += 1;
                    }
                }
                // left down
                if (x + 3) < grid_len && y > 2 {
                    let word = [
                        grid[x][y],
                        grid[x + 1][y - 1],
                        grid[x + 2][y - 2],
                        grid[x + 3][y - 3],
                    ];
                    let word: String = word.iter().collect();
                    if word == "XMAS" {
                        count += 1;
                    }
                }
                // right up
                if x > 2 && y < row_len - 3 {
                    let word = [
                        grid[x][y],
                        grid[x - 1][y + 1],
                        grid[x - 2][y + 2],
                        grid[x - 3][y + 3],
                    ];
                    let word: String = word.iter().collect();
                    if word == "XMAS" {
                        count += 1;
                    }
                }
                // right down
                if x + 3 < grid_len && y < row_len - 3 {
                    let word = [
                        grid[x][y],
                        grid[x + 1][y + 1],
                        grid[x + 2][y + 2],
                        grid[x + 3][y + 3],
                    ];
                    let word: String = word.iter().collect();
                    if word == "XMAS" {
                        count += 1;
                    }
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

        assert_eq!(18, result);
    }

    #[test]
    fn file() {
        dotenvy::dotenv().expect("should be able to load .env file!");

        let result = solve_day4_puzzle1();

        assert_eq!(2536, result);
    }
}
