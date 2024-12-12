use std::collections::{HashMap, HashSet};

use crate::read_file;

pub fn solve_puzzle1() -> (i64, i64) {
    get_result(read_file("day12.txt").as_str())
}

fn get_result(content: &str) -> (i64, i64) {
    let mut sum_by_perimeter = 0;
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
            walk(&grid, pos, *plant_type, &mut region, &mut region_perimeter);
            sum_by_perimeter += region.len() as i64 * region_perimeter.values().sum::<i64>();

            for p in region {
                visited.insert(p);
            }
        }
    }
    (sum_by_perimeter, 0)
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
}

fn walk(
    grid: &Grid,
    pos: Pos,
    plant_type: char,
    visited: &mut HashSet<Pos>,
    perimeters: &mut HashMap<Pos, i64>,
) -> bool {
    let current_char = grid[pos.x][pos.y];
    if current_char != plant_type {
        return false;
    }

    if visited.contains(&pos) {
        return true;
    }

    visited.insert(pos);

    let up = pos.up();
    let right = pos.right(grid[pos.x].len());
    let down = pos.down(grid.len());
    let left = pos.left();

    if let Some(up) = up {
        // try going up
        if !walk(grid, up, plant_type, visited, perimeters) {
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
        if !walk(grid, right, plant_type, visited, perimeters) {
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
        if !walk(grid, down, plant_type, visited, perimeters) {
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
        if !walk(grid, left, plant_type, visited, perimeters) {
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
        // assert_eq!(1206, result.1);
    }

    #[test]
    fn file_puzzle1() {
        dotenvy::dotenv().expect("should be able to load .env file!");

        let result = solve_puzzle1();

        assert_eq!(1452678, result.0);
    }
}
