use std::collections::{HashMap, HashSet};

use crate::read_file;

pub fn solve_puzzle1() -> (i64, i64) {
    get_result(read_file("day12.txt").as_str())
}

fn get_result(content: &str) -> (i64, i64) {
    let mut sum_by_perimeter = 0;
    let mut sum_by_corners = 0;
    let grid = parse_chars(content);
    let mut visited: HashSet<Pos> = HashSet::new();
    for (x, row) in grid.iter().enumerate() {
        for (y, plant_type) in row.iter().enumerate() {
            let pos = Pos { x, y };
            if visited.contains(&pos) {
                continue;
            }

            let mut region = HashSet::new();
            let mut region_perimeter = HashMap::new();
            let mut corner_count = 0;
            walk(
                &grid,
                pos,
                *plant_type,
                &mut region,
                &mut region_perimeter,
                &mut corner_count,
            );
            sum_by_perimeter += region.len() as i64 * region_perimeter.values().sum::<i64>();
            sum_by_corners += region.len() as i64 * corner_count;

            for p in region {
                visited.insert(p);
            }
        }
    }
    (sum_by_perimeter, sum_by_corners)
}

type Grid = Vec<Vec<char>>;

fn parse_chars(content: &str) -> Grid {
    content.lines().map(|l| l.chars().collect()).collect()
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn up(&self) -> Option<Pos> {
        if self.x > 0 {
            return Some(Pos {
                x: self.x - 1,
                y: self.y,
            });
        }

        None
    }

    fn right(&self, column_len: usize) -> Option<Pos> {
        if self.y < column_len - 1 {
            return Some(Pos {
                x: self.x,
                y: self.y + 1,
            });
        }

        None
    }

    fn down(&self, row_len: usize) -> Option<Pos> {
        if self.x < row_len - 1 {
            return Some(Pos {
                x: self.x + 1,
                y: self.y,
            });
        }

        None
    }

    fn left(&self) -> Option<Pos> {
        if self.y > 0 {
            return Some(Pos {
                x: self.x,
                y: self.y - 1,
            });
        }

        None
    }

    fn left_up(&self) -> Option<Pos> {
        if let Some(left) = self.left() {
            left.up()
        } else {
            None
        }
    }

    fn left_down(&self, row_len: usize) -> Option<Pos> {
        if let Some(left) = self.left() {
            left.down(row_len)
        } else {
            None
        }
    }

    fn right_up(&self, column_len: usize) -> Option<Pos> {
        if let Some(right) = self.right(column_len) {
            right.up()
        } else {
            None
        }
    }

    fn right_down(&self, column_len: usize, row_len: usize) -> Option<Pos> {
        if let Some(right) = self.right(column_len) {
            right.down(row_len)
        } else {
            None
        }
    }
}

fn count_corners(grid: &Grid, center: &Pos) -> i64 {
    let plant = grid[center.x][center.y];
    let column_len = grid[center.x].len();
    let row_len = grid.len();

    let left_up = center.left_up();
    let up = center.up();
    let right_up = center.right_up(column_len);
    let right = center.right(column_len);
    let right_down = center.right_down(column_len, row_len);
    let down = center.down(row_len);
    let left_down = center.left_down(row_len);
    let left = center.left();

    let nw = get_value(grid, left_up, plant);
    let n = get_value(grid, up, plant);
    let ne = get_value(grid, right_up, plant);
    let e = get_value(grid, right, plant);
    let se = get_value(grid, right_down, plant);
    let s = get_value(grid, down, plant);
    let sw = get_value(grid, left_down, plant);
    let w = get_value(grid, left, plant);

    let mut count = 0;

    // single corners
    if !w && !n {
        count += 1;
    }
    if !n && !e {
        count += 1;
    }
    if !e && !s {
        count += 1;
    }
    if !s && !w {
        count += 1;
    }

    if !ne & n & e {
        count += 1;
    }

    if !se & e & s {
        count += 1;
    }

    if !sw & w & s {
        count += 1;
    }

    if !nw & w & n {
        count += 1;
    }

    count
}

fn get_value(grid: &Grid, pos: Option<Pos>, plant: char) -> bool {
    if pos.is_some() {
        let left_up = pos.unwrap();
        let value = grid[left_up.x][left_up.y];
        value == plant
    } else {
        false
    }
}

fn walk(
    grid: &Grid,
    pos: Pos,
    plant_type: char,
    visited: &mut HashSet<Pos>,
    perimeters: &mut HashMap<Pos, i64>,
    corner_count: &mut i64,
) -> bool {
    let current_char = grid[pos.x][pos.y];
    if current_char != plant_type {
        return false;
    }

    if visited.contains(&pos) {
        return true;
    }

    visited.insert(pos);

    let column_len = grid[pos.x].len();
    let row_len = grid.len();

    let up = pos.up();
    let right = pos.right(column_len);
    let down = pos.down(row_len);
    let left = pos.left();

    *corner_count += count_corners(grid, &pos);

    if let Some(up) = up {
        // try going up
        if !walk(grid, up, plant_type, visited, perimeters, corner_count) {
            let entry = perimeters.entry(pos).or_default();
            *entry += 1;
        }
    } else {
        // then we know there is a fence needed
        let entry = perimeters.entry(pos).or_default();
        *entry += 1;
    }

    if let Some(right) = right {
        // try going right
        if !walk(grid, right, plant_type, visited, perimeters, corner_count) {
            let entry = perimeters.entry(pos).or_default();
            *entry += 1;
        }
    } else {
        // then we know there is a fence needed
        let entry = perimeters.entry(pos).or_default();
        *entry += 1;
    }

    if let Some(down) = down {
        // try going down
        if !walk(grid, down, plant_type, visited, perimeters, corner_count) {
            let entry = perimeters.entry(pos).or_default();
            *entry += 1;
        }
    } else {
        // then we know there is a fence needed
        let entry = perimeters.entry(pos).or_default();
        *entry += 1;
    }

    if let Some(left) = left {
        // try going left
        if !walk(grid, left, plant_type, visited, perimeters, corner_count) {
            let entry = perimeters.entry(pos).or_default();
            *entry += 1;
        }
    } else {
        // then we know there is a fence needed
        let entry = perimeters.entry(pos).or_default();
        *entry += 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let content = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

        let result = get_result(content);

        assert_eq!(1930, result.0);
        assert_eq!(1206, result.1);
    }

    #[test]
    fn file() {
        dotenvy::dotenv().expect("should be able to load .env file!");

        let result = solve_puzzle1();

        assert_eq!(1452678, result.0);
        assert_eq!(873584, result.1);
    }
}
