use std::borrow::BorrowMut;

pub mod puzzle1;
pub mod puzzle2;

struct Map {
    tiles: Vec<Vec<Tile>>,
    guard: Guard,
}

impl Map {
    fn parse(content: &str) -> Self {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        let mut guard: Option<Guard> = None;

        for (x, line) in content.lines().enumerate() {
            let mut tile_line: Vec<Tile> = Vec::new();
            for (y, c) in line.chars().enumerate() {
                let mut tile = Tile::parse(c);
                if c == '^' {
                    guard = Some(Guard {
                        pos_x: x,
                        pos_y: y,
                        direction: Direction::Up,
                    });
                    tile.visited = true;
                }
                tile_line.push(tile);
            }

            tiles.push(tile_line);
        }
        Self {
            tiles,
            guard: guard.expect("guard was not found!"),
        }
    }

    fn move_guard(&mut self) -> bool {
        let exit_x = self.tiles.len() - 1;
        let exit_y = self.tiles[self.guard.pos_x].len() - 1;

        let next_pos: Option<(usize, usize)> = match self.guard.direction {
            Direction::Up => {
                if self.guard.pos_x == 0 {
                    None
                } else {
                    Some((self.guard.pos_x - 1, self.guard.pos_y))
                }
            }
            Direction::Down => {
                if self.guard.pos_x == exit_x {
                    None
                } else {
                    Some((self.guard.pos_x + 1, self.guard.pos_y))
                }
            }
            Direction::Left => {
                if self.guard.pos_y == 0 {
                    None
                } else {
                    Some((self.guard.pos_x, self.guard.pos_y - 1))
                }
            }
            Direction::Right => {
                if self.guard.pos_y == exit_y {
                    None
                } else {
                    Some((self.guard.pos_x, self.guard.pos_y + 1))
                }
            }
        };

        if let Some((next_x, next_y)) = next_pos {
            let tile = self.tiles[next_x][next_y].borrow_mut();
            match tile.tile_type {
                TileType::Open => {
                    self.guard.pos_x = next_x;
                    self.guard.pos_y = next_y;
                    tile.visited = true;
                }
                TileType::Obstacle => self.guard.turn_right(),
            }

            true
        } else {
            false
        }
    }
}

#[derive(PartialEq)]
enum TileType {
    Open,
    Obstacle,
}

struct Tile {
    tile_type: TileType,
    visited: bool,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Guard {
    pos_x: usize,
    pos_y: usize,
    direction: Direction,
}

impl Guard {
    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };
    }
}

impl Tile {
    fn parse(value: char) -> Self {
        let tile_type = if value == '#' {
            TileType::Obstacle
        } else {
            TileType::Open
        };
        Self {
            visited: false,
            tile_type,
        }
    }
}
