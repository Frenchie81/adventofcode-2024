use crate::read_file;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub fn solve_puzzle_1() -> i64 {
    let content = read_file("day16.txt");
    get_result(&content)
}

fn get_result(content: &str) -> i64 {
    let (grid, start_pos, end_pos) = parse_content(content);
    let mut visit_results: HashMap<Pos, VisitResult> = HashMap::new();
    let mut visit_queue = PriorityQueue::new();
    let mut visited = HashSet::new();
    visit_queue.add(Visit {
        pos: start_pos,
        direction: Direction::East,
        cost_from_start: 0,
    });
    while let Some(visit) = visit_queue.dequeue() {
        if let Some(forward_tile) = try_get_next_tile(&grid, &visit.pos, &visit.direction) {
            let new_cost = visit.cost_from_start + 1;
            update_visit_result(&mut visit_results, &forward_tile.pos, &visit.pos, new_cost);
            if !visited.contains(&forward_tile.pos) {
                visit_queue.add(Visit {
                    cost_from_start: new_cost,
                    pos: forward_tile.pos,
                    direction: visit.direction,
                });
            }
        }

        let left = visit.direction.turn_left();
        if let Some(left_tile) = try_get_next_tile(&grid, &visit.pos, &visit.direction.turn_left())
        {
            let new_cost = visit.cost_from_start + 1001;
            update_visit_result(&mut visit_results, &left_tile.pos, &visit.pos, new_cost);
            if !visited.contains(&left_tile.pos) {
                visit_queue.add(Visit {
                    cost_from_start: new_cost,
                    pos: left_tile.pos,
                    direction: left,
                });
            }
        }

        let right = visit.direction.turn_right();
        if let Some(right_tile) =
            try_get_next_tile(&grid, &visit.pos, &visit.direction.turn_right())
        {
            let new_cost = visit.cost_from_start + 1001;
            update_visit_result(&mut visit_results, &right_tile.pos, &visit.pos, new_cost);
            if !visited.contains(&right_tile.pos) {
                visit_queue.add(Visit {
                    cost_from_start: new_cost,
                    pos: right_tile.pos,
                    direction: right,
                });
            }
        }

        visited.insert(visit.pos);
    }

    if let Some(end_visit) = visit_results.get(&end_pos) {
        end_visit.min_cost
    } else {
        0
    }
}

fn update_visit_result(
    visit_results: &mut HashMap<Pos, VisitResult>,
    pos: &Pos,
    from: &Pos,
    cost: i64,
) {
    visit_results
        .entry(*pos)
        .and_modify(|v| {
            if v.min_cost > cost {
                v.min_cost = cost;
                v.from = *from;
            }
        })
        .or_insert(VisitResult {
            from: *from,
            min_cost: cost,
        });
}

fn parse_content(content: &str) -> (Grid, Pos, Pos) {
    let mut grid = Grid::new();
    let mut start_pos: Option<Pos> = None;
    let mut end_pos: Option<Pos> = None;
    for (y, line) in content.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let tile_type = match c {
                '#' => TileType::Wall,
                'S' => {
                    start_pos = Some(Pos { x, y });
                    TileType::Start
                }
                'E' => {
                    end_pos = Some(Pos { x, y });
                    TileType::End
                }
                _ => TileType::Open,
            };
            row.push(Tile {
                tile_type,
                pos: Pos { x, y },
            });
        }
        grid.push(row);
    }
    (grid, start_pos.unwrap(), end_pos.unwrap())
}

#[derive(Debug)]
struct VisitResult {
    min_cost: i64,
    from: Pos,
}

#[derive(Debug, Copy, Clone)]
struct Visit {
    pos: Pos,
    direction: Direction,
    cost_from_start: i64,
}

type Grid = Vec<Vec<Tile>>;

struct PriorityQueue {
    list: Vec<Visit>,
}

impl PriorityQueue {
    fn new() -> Self {
        Self { list: Vec::new() }
    }

    fn add(&mut self, visit: Visit) {
        self.list.push(visit);
        self.list
            .sort_by(|a, b| b.cost_from_start.cmp(&a.cost_from_start));
    }

    fn dequeue(&mut self) -> Option<Visit> {
        self.list.pop()
    }
}

fn try_get_next_tile(grid: &Grid, pos: &Pos, direction: &Direction) -> Option<Tile> {
    let tile = match direction {
        Direction::North => {
            if pos.y < 1 {
                return None;
            } else {
                grid[pos.y - 1][pos.x]
            }
        }
        Direction::East => {
            let row = &grid[pos.y];
            if pos.x + 1 > row.len() {
                return None;
            } else {
                row[pos.x + 1]
            }
        }
        Direction::South => {
            if pos.y + 1 > grid.len() {
                return None;
            } else {
                grid[pos.y + 1][pos.x]
            }
        }
        Direction::West => {
            if pos.x < 1 {
                return None;
            } else {
                grid[pos.y][pos.x - 1]
            }
        }
    };

    match tile.tile_type {
        TileType::Wall => None,
        _ => Some(tile),
    }
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    tile_type: TileType,
    pos: Pos,
}

#[derive(Debug, Copy, Clone)]
enum TileType {
    Wall,
    Start,
    End,
    Open,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.tile_type {
            TileType::Wall => '#',
            TileType::Start => 'S',
            TileType::End => 'E',
            TileType::Open => '.',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
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

    #[test]
    fn file() {
        dotenvy::dotenv().expect("should be able to load .env file!");

        let result = solve_puzzle_1();

        assert_eq!(99448, result);
    }
}
