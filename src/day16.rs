use std::{collections::HashSet, os::unix::thread, time::Duration};

use chrono::Utc;

use crate::read_file;

pub fn solve_puzzle_1() -> i64 {
    let content = read_file("day16.txt");
    get_result(&content)
}

fn get_result(content: &str) -> i64 {
    let mut grid = parse_maze(content);
    let start = find_start(&grid).expect("there should always be a start tile");
    solve_maze(&mut grid, start)
}

fn parse_maze(content: &str) -> Grid {
    let mut grid = Grid::new();
    for line in content.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    grid
}

fn render_grid(grid: &Grid, current_pos: &Pos, direction: &Direction) {
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if current_pos.x == x && current_pos.y == y {
                let d = match direction {
                    Direction::North => '^',
                    Direction::East => '>',
                    Direction::South => 'v',
                    Direction::West => '<',
                };
                print!("{d}");
            } else {
                print!("{c}");
            }
        }
        println!();
    }
}

fn find_start(grid: &Grid) -> Option<Pos> {
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                return Some(Pos::new(x, y));
            }
        }
    }

    None
}

fn next_open_pos(grid: &Grid, current_pos: &Pos, direction: &Direction) -> Option<Pos> {
    let max_y = grid.len();
    let max_x = grid.first().expect("should always be one row").len();

    let pos = match direction {
        Direction::North => {
            if current_pos.y == 0 {
                None
            } else {
                Some(Pos::new(current_pos.x, current_pos.y - 1))
            }
        }
        Direction::East => {
            if current_pos.x >= max_x {
                None
            } else {
                Some(Pos::new(current_pos.x + 1, current_pos.y))
            }
        }
        Direction::South => {
            if current_pos.y >= max_y {
                None
            } else {
                Some(Pos::new(current_pos.x, current_pos.y + 1))
            }
        }
        Direction::West => {
            if current_pos.x == 0 {
                None
            } else {
                Some(Pos::new(current_pos.x - 1, current_pos.y))
            }
        }
    };

    pos?;
    let pos = pos.unwrap();

    let c = grid[pos.y][pos.x];
    if c == '#' {
        return None;
    }

    Some(pos)
}

fn solve_maze(grid: &mut Grid, starting_pos: Pos) -> i64 {
    let mut visited = HashSet::new();
    let mut scores = HashSet::new();

    render_grid(grid, &starting_pos, &Direction::East);

    recurse(
        grid,
        &starting_pos,
        Direction::East,
        &mut visited,
        0,
        &mut scores,
    );

    *scores
        .iter()
        .min()
        .expect("there should always be at least one score")
}

fn recurse(
    grid: &mut Grid,
    pos: &Pos,
    direction: Direction,
    visited: &mut HashSet<Pos>,
    current_score: i64,
    scores: &mut HashSet<i64>,
) {
    let row = grid.get(pos.y);
    if row.is_none() {
        return;
    }
    let row = row.unwrap();

    let tile = row.get(pos.x);
    if tile.is_none() {
        return;
    }
    let tile = tile.unwrap();

    // println!("{pos:?}");
    // println!("{scores:?}");
    // std::thread::sleep(Duration::from_millis(10));
    if visited.contains(pos) {
        return;
    }

    visited.insert(*pos);

    // render_grid(grid, pos, &direction);
    // std::thread::sleep(Duration::from_millis(500));

    if *tile == 'E' {
        let now = Utc::now();
        println!("{now} win! {current_score}");
        scores.insert(current_score);
        return;
    }

    // try to move
    if let Some(next_pos) = next_open_pos(grid, pos, &direction) {
        recurse(
            grid,
            &next_pos,
            direction,
            &mut visited.clone(),
            current_score + 1,
            scores,
        );
    }

    // turn left
    let left = direction.turn_left();
    if let Some(next_pos) = next_open_pos(grid, pos, &left) {
        recurse(
            grid,
            &next_pos,
            left,
            // &mut visited.clone(),
            visited,
            current_score + 1000 + 1,
            scores,
        );
    }

    // turn right
    let right = direction.turn_right();
    if let Some(next_pos) = next_open_pos(grid, pos, &direction.turn_right()) {
        recurse(
            grid,
            &next_pos,
            right,
            visited,
            // &mut visited.clone(),
            current_score + 1000 + 1,
            scores,
        );
    }
}

#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

type Grid = Vec<Vec<char>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let content = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

        let result = get_result(content);

        assert_eq!(7036, result);
    }

    #[test]
    fn demo2() {
        let content = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

        let result = get_result(content);

        assert_eq!(11048, result);
    }
}
