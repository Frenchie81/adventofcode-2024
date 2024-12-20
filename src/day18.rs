use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};

use crate::read_file;

pub fn solve_puzzle_1() -> i64 {
    let content = read_file("day18.txt");
    get_result(&content, 71, 1024)
}

fn get_result(content: &str, grid_size: usize, num_bytes: usize) -> i64 {
    let mut corrupted_pos_list = parse_content(content);
    corrupted_pos_list.truncate(num_bytes);
    let grid = generate_grid(grid_size, &corrupted_pos_list);
    render_grid(&grid);

    find_lowest_cost(&grid, grid_size)
}

fn parse_content(content: &str) -> Vec<Pos> {
    let mut result = Vec::new();
    for line in content.lines() {
        let splits: Vec<&str> = line.split(",").collect();
        let x = splits.first().unwrap().parse().unwrap();
        let y = splits.get(1).unwrap().parse().unwrap();
        result.push(Pos::new(x, y));
    }

    result
}

type Grid = Vec<Vec<Tile>>;

fn generate_grid(grid_size: usize, corrupted_pos_list: &[Pos]) -> Grid {
    let mut grid = Grid::new();
    (0..grid_size).for_each(|y| {
        let mut row = Vec::new();
        (0..grid_size).for_each(|x| {
            let pos = Pos::new(x, y);
            let tile = if corrupted_pos_list.contains(&pos) {
                Tile {
                    tile_type: TileType::Corrupted,
                    pos,
                }
            } else {
                Tile {
                    tile_type: TileType::Open,
                    pos,
                }
            };
            row.push(tile);
        });

        grid.push(row);
    });

    grid
}

fn render_grid(grid: &Grid) {
    for row in grid.iter() {
        for tile in row.iter() {
            print!("{}", tile.tile_type);
        }
        println!();
    }
}

fn find_lowest_cost(grid: &Grid, grid_size: usize) -> i64 {
    let start_pos = Pos::new(0, 0);
    let end_pos = Pos::new(grid_size - 1, grid_size - 1);

    let mut visited: HashSet<Pos> = HashSet::new();
    let mut visit_list: HashMap<Pos, VisitEntry> = HashMap::new();
    let mut visit_queue = PriorityQueue::new();
    visit_queue.add(Visit::new(start_pos, 0));

    while let Some(visit) = visit_queue.dequeue() {
        if visited.contains(&visit.pos) {
            continue;
        }

        let mut neighbors = Vec::new();
        if visit.pos.x > 0 {
            let left = grid[visit.pos.y][visit.pos.x - 1];
            neighbors.push(left);
        }
        if visit.pos.y > 0 {
            let up = grid[visit.pos.y - 1][visit.pos.x];
            neighbors.push(up);
        }
        if visit.pos.x < grid_size - 1 {
            let right = grid[visit.pos.y][visit.pos.x + 1];
            neighbors.push(right);
        }
        if visit.pos.y < grid_size - 1 {
            let down = grid[visit.pos.y + 1][visit.pos.x];
            neighbors.push(down);
        }

        for neighbor in neighbors {
            if visited.contains(&neighbor.pos) {
                continue;
            }

            match neighbor.tile_type {
                TileType::Open => {
                    update_visit_entry(
                        &mut visit_list,
                        neighbor.pos,
                        visit.pos,
                        visit.cost_from_start + 1,
                    );
                    visit_queue.add(Visit::new(neighbor.pos, visit.cost_from_start + 1))
                }
                TileType::Corrupted => (),
            }
        }

        visited.insert(visit.pos);
    }

    if let Some(finish) = visit_list.get(&end_pos) {
        finish.min_cost
    } else {
        0
    }
}

fn update_visit_entry(
    visits: &mut HashMap<Pos, VisitEntry>,
    pos: Pos,
    from: Pos,
    cost_from_start: i64,
) {
    visits
        .entry(pos)
        .and_modify(|v| {
            if v.min_cost > cost_from_start {
                v.min_cost = cost_from_start;
            }
        })
        .or_insert(VisitEntry {
            pos,
            from,
            min_cost: cost_from_start,
        });
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Visit {
    pos: Pos,
    cost_from_start: i64,
}

impl Visit {
    fn new(pos: Pos, cost_from_start: i64) -> Self {
        Self {
            pos,
            cost_from_start,
        }
    }
}

struct VisitEntry {
    pos: Pos,
    min_cost: i64,
    from: Pos,
}

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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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
enum TileType {
    Open,
    Corrupted,
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            TileType::Open => '.',
            TileType::Corrupted => '#',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Tile {
    tile_type: TileType,
    pos: Pos,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let content = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

        let result = get_result(content, 7, 12);

        assert_eq!(22, result);
    }

    #[test]
    fn file() {
        dotenvy::dotenv().expect("should be able to load .env file!");

        let result = solve_puzzle_1();

        assert_eq!(292, result);
    }
}
