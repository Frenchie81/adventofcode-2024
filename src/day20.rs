use crate::read_file;

pub fn solve_puzzle_1() -> i64 {
    let content = read_file("day20.txt");
    get_result(&content)
}

fn get_result(content: &str) -> i64 {
    let (grid, start) = parse_content(content);

    let baseline = run_race(&grid, start, Direction::North);

    baseline.unwrap_or_default()
}

fn parse_content(content: &str) -> (Grid, Pos) {
    let mut grid = Grid::new();
    let mut start = Pos::new(0, 0);
    for (y, line) in content.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = Pos::new(x, y);
            }
            row.push(c);
        }

        grid.push(row);
    }

    (grid, start)
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn get_next_pos(&self, grid: &Grid, direction: Direction) -> Option<Pos> {
        let y_len = grid.len();
        let x_len = grid.first()?.len();

        match direction {
            Direction::North => {
                if self.y == 0 {
                    None
                } else {
                    Some(Pos::new(self.x, self.y - 1))
                }
            }
            Direction::East => {
                if self.x >= x_len {
                    None
                } else {
                    Some(Pos::new(self.x + 1, self.y))
                }
            }
            Direction::South => {
                if self.y >= y_len {
                    None
                } else {
                    Some(Pos::new(self.x, self.y + 1))
                }
            }
            Direction::West => {
                if self.x == 0 {
                    None
                } else {
                    Some(Pos::new(self.x - 1, self.y))
                }
            }
        }
    }
}

type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone, Copy)]
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
            Direction::North => Self::East,
            Direction::East => Self::South,
            Direction::South => Self::West,
            Direction::West => Self::North,
        }
    }
}

fn run_race(grid: &Grid, cur_pos: Pos, cur_direction: Direction) -> Option<i64> {
    let row = grid.get(cur_pos.y)?;
    let tile = row.get(cur_pos.x)?;

    if *tile == 'E' {
        return Some(0);
    }

    if *tile == '#' {
        return None;
    }

    if let Some(forward) = cur_pos.get_next_pos(grid, cur_direction) {
        if let Some(c) = run_race(grid, forward, cur_direction) {
            return Some(c + 1);
        }

        let turn_left = cur_direction.turn_left();
        if let Some(left) = cur_pos.get_next_pos(grid, turn_left) {
            if let Some(c) = run_race(grid, left, turn_left) {
                return Some(c + 1);
            }
        }

        let turn_right = cur_direction.turn_right();
        if let Some(right) = cur_pos.get_next_pos(grid, turn_right) {
            if let Some(c) = run_race(grid, right, turn_right) {
                return Some(c + 1);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let content = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

        let result = get_result(content);

        // saves at least 10 picoseconds
        assert_eq!(10, result);
    }
}
