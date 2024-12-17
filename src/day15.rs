use std::collections::VecDeque;

use crate::read_file;

pub fn solve_puzzle_1() -> i64 {
    let content = read_file("day15.txt");
    get_result(&content)
}

fn get_result(content: &str) -> i64 {
    let (mut warehouse, mut moves) = parse_content(content);
    let robot_pos = warehouse.find_robot();
    robot_pos.expect("should always start with a robot");

    let mut next_pos = robot_pos.unwrap();
    while let Some(direction) = moves.pop_front() {
        next_pos = if let Some(p) = try_move(next_pos, &mut warehouse, direction) {
            p
        } else {
            next_pos
        };
    }
    warehouse.render();
    warehouse.sum_box_gps_locations()
}

fn try_move(pos: Pos, warehouse: &mut Warehouse, direction: Direction) -> Option<Pos> {
    let row = warehouse.floor.get(pos.y);
    row?;

    let row = row.unwrap();
    let tile = row.get(pos.x);
    tile?;

    let tile = tile.unwrap();
    let tile2 = *tile;
    match tile {
        Tile::Wall => None,
        Tile::Open => Some(pos),
        _ => {
            let next = pos.move_direction(&direction);
            if try_move(next, warehouse, direction).is_some() {
                warehouse.floor[next.y][next.x] = tile2;
                warehouse.floor[pos.y][pos.x] = Tile::Open;
                Some(next)
            } else {
                None
            }
        }
    }
}

fn parse_content(content: &str) -> (Warehouse, VecDeque<Direction>) {
    let mut warehouse = Warehouse { floor: Grid::new() };
    let mut moves = VecDeque::new();

    let mut switch = false;

    for line in content.lines() {
        if switch {
            for c in line.chars() {
                let direction = match c {
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    '^' => Direction::Up,
                    _ => Direction::Down,
                };

                moves.push_back(direction);
            }
        } else if line.is_empty() {
            switch = true;
        } else {
            let mut row = Vec::new();

            for c in line.chars() {
                let tile = match c {
                    '#' => Tile::Wall,
                    '@' => Tile::Robot,
                    'O' => Tile::Box,
                    _ => Tile::Open,
                };

                row.push(tile);
            }

            warehouse.floor.push(row);
        }
    }

    (warehouse, moves)
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Wall,
    Open,
    Robot,
    Box,
}

#[derive(Debug, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn move_direction(&self, direction: &Direction) -> Pos {
        match direction {
            Direction::Up => {
                if self.y == 0 {
                    *self
                } else {
                    Pos {
                        x: self.x,
                        y: self.y - 1,
                    }
                }
            }
            Direction::Right => Pos {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Pos {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => {
                if self.x == 0 {
                    *self
                } else {
                    Pos {
                        x: self.x - 1,
                        y: self.y,
                    }
                }
            }
        }
    }
}

type Grid = Vec<Vec<Tile>>;

struct Warehouse {
    floor: Grid,
}

impl Warehouse {
    fn find_robot(&self) -> Option<Pos> {
        for (y, row) in self.floor.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Robot => return Some(Pos { x, y }),
                    _ => continue,
                }
            }
        }

        None
    }

    fn render(&self) {
        for row in self.floor.iter() {
            for tile in row.iter() {
                let c = match tile {
                    Tile::Wall => '#',
                    Tile::Open => '.',
                    Tile::Robot => '@',
                    Tile::Box => 'O',
                };

                print!("{c}");
            }
            println!();
        }
    }

    fn sum_box_gps_locations(&self) -> i64 {
        let mut sum = 0;
        for (y, row) in self.floor.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let coord = match tile {
                    Tile::Box => 100 * y + x,
                    _ => 0,
                };

                sum += coord as i64;
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let content = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        let result = get_result(content);

        assert_eq!(10092, result);
    }

    #[test]
    fn file() {
        dotenvy::dotenv().expect("should be able to load .env file!");

        let result = solve_puzzle_1();

        assert_eq!(1349898, result);
    }
}
