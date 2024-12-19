use crate::read_file;
use std::{
    collections::HashSet,
    fmt::{format, Display},
    hash::Hash,
    time::Duration,
};

pub fn solve_puzzle_1() -> i64 {
    let content = read_file("day16.txt");
    get_result(&content)
}

fn get_result(content: &str) -> i64 {
    let grid = parse_maze(content);

    let start = find_start(&grid).expect("there should always be a start tile");
    let starting_bot = Robot {
        pos: start,
        score: 0,
        direction: Direction::East,
        memory: HashSet::new(),
        finished: false,
        blocked: false,
        path: HashSet::new(),
    };

    let mut maze = Maze {
        grid,
        robots: vec![starting_bot],
        visited: HashSet::new(),
    };

    maze.run();
    maze.get_lowest_score()
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

fn next_open_pos(
    grid: &Grid,
    current_pos: &Pos,
    direction: &Direction,
    memory: &HashSet<Pos>,
) -> Option<Pos> {
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

    if memory.contains(&pos) {
        return None;
    }

    let c = grid[pos.y][pos.x];
    if c == '#' {
        return None;
    }

    Some(pos)
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

#[derive(Debug, Clone)]
struct Robot {
    pos: Pos,
    direction: Direction,
    memory: HashSet<Pos>,
    score: i64,
    blocked: bool,
    finished: bool,
    path: HashSet<Pos>,
}

impl Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Robot Pos: {:?}, Blocked: {}, Finished: {}, score: {}",
            self.pos, self.blocked, self.finished, self.score
        )
    }
}

struct Maze {
    grid: Grid,
    robots: Vec<Robot>,
    visited: HashSet<Pos>,
}

impl Maze {
    fn render_winner(&self) {
        let low_score = self.get_lowest_score();
        if low_score < 0 {
            return;
        }

        let winners: Vec<&Robot> = self
            .robots
            .iter()
            .filter(|r| r.score == low_score)
            .collect();

        for (y, row) in self.grid.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let mut c = *tile;
                let pos = Pos { x, y };
                for (i, r) in winners.iter().enumerate() {
                    if r.path.contains(&pos) {
                        c = *i
                            .to_string()
                            .chars()
                            .collect::<Vec<char>>()
                            .first()
                            .unwrap();
                    }
                }
                print!("{c}");
            }
            println!();
        }
        self.robots.iter().for_each(|r| {
            println!("{r}");
        });
    }

    fn render(&self) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let mut c = *tile;
                let pos = Pos { x, y };
                if self.visited.contains(&pos) {
                    c = 'O';
                }
                if let Some(robot) = self.robots.iter().find(|r| r.pos.x == x && r.pos.y == y) {
                    c = match robot.direction {
                        Direction::North => '^',
                        Direction::East => '>',
                        Direction::South => 'v',
                        Direction::West => '<',
                    }
                }
                print!("{c}");
            }
            println!();
        }
        self.robots.iter().for_each(|r| {
            println!("{r}");
        });
    }

    fn run(&mut self) {
        while self.robots.iter().any(|r| !(r.finished)) {
            self.tick();
            // remove any blocked robots
            self.robots.retain(|r| !r.blocked);
            // self.render();
            // std::thread::sleep(Duration::from_millis(1000));
        }

        self.render_winner();
    }

    fn tick(&mut self) {
        let mut spawns = Vec::new();
        for robot in self.robots.iter_mut().filter(|r| !r.finished && !r.blocked) {
            // set the robots memory to start
            robot.memory.insert(robot.pos);
            self.visited.insert(robot.pos);

            let left_direction = robot.direction.turn_left();
            let left_pos = next_open_pos(&self.grid, &robot.pos, &left_direction, &robot.memory);
            if left_pos.is_some() {
                let mut new_robot = robot.clone();
                new_robot.direction = left_direction;
                new_robot.score += 1000;
                spawns.push(new_robot);
            }

            let right_direction = robot.direction.turn_right();
            let right_pos = next_open_pos(&self.grid, &robot.pos, &right_direction, &robot.memory);
            if right_pos.is_some() {
                let mut new_robot = robot.clone();
                new_robot.direction = right_direction;
                new_robot.score += 1000;
                spawns.push(new_robot);
            }

            if let Some(next_pos) =
                next_open_pos(&self.grid, &robot.pos, &robot.direction, &robot.memory)
            {
                let tile = self.grid[next_pos.y][next_pos.x];
                robot.score += 1;
                if tile == 'E' {
                    robot.finished = true;
                } else {
                    robot.path.insert(next_pos);
                    robot.pos = next_pos;
                    for spawn in spawns.iter_mut() {
                        spawn.memory.insert(next_pos);
                    }
                }
            } else {
                robot.blocked = true;
            }
        }

        for spawn in spawns {
            self.robots.push(spawn);
        }
    }

    fn get_lowest_score(&self) -> i64 {
        let mut lowest_score = -1;

        for r in self.robots.iter().filter(|r| r.finished) {
            if lowest_score == -1 || r.score < lowest_score {
                lowest_score = r.score;
            }
        }

        lowest_score
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
    fn demo1() {
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
