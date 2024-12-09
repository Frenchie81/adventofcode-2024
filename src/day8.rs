use std::collections::HashMap;

pub mod puzzle1;
pub mod puzzle2;

#[derive(Debug, Copy, Clone)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn difference(&self, pos: Pos) -> Pos {
        Pos {
            x: self.x - pos.x,
            y: self.y - pos.y,
        }
    }
}

enum TileType {
    Open,
    Tower(char),
}

struct Tile {
    tile_type: TileType,
    antinodes: Vec<char>,
}

impl Tile {
    fn parse(c: char) -> Self {
        let tile_type = if c == '.' {
            TileType::Open
        } else {
            TileType::Tower(c)
        };

        Self {
            tile_type,
            antinodes: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Frequency {
    positions: Vec<Pos>,
}

type Grid = Vec<Vec<Tile>>;

struct Map {
    grid: Grid,
    frequencies: HashMap<char, Frequency>,
}

impl Map {
    fn parse(content: &str) -> Self {
        let mut grid = Grid::new();

        for line in content.lines() {
            let mut row: Vec<Tile> = Vec::new();
            for c in line.chars() {
                row.push(Tile::parse(c));
            }
            grid.push(row);
        }

        let mut frequencies: HashMap<char, Frequency> = HashMap::new();
        for (x, row) in grid.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                if let TileType::Tower(c) = tile.tile_type {
                    let frequency = frequencies.entry(c).or_insert(Frequency {
                        positions: Vec::new(),
                    });

                    let pos = Pos {
                        x: x as i64,
                        y: y as i64,
                    };
                    frequency.positions.push(pos);
                }
            }
        }

        Self { grid, frequencies }
    }

    fn apply_frequencies(&mut self) {
        for (k, v) in self.frequencies.iter() {
            let positions_len = v.positions.len();
            for (i, pos1) in v.positions.iter().enumerate() {
                for j in i + 1..positions_len {
                    let pos2 = v.positions[j];
                    let diff = pos1.difference(pos2);

                    let antinode1 = Pos {
                        x: pos1.x + diff.x,
                        y: pos1.y + diff.y,
                    };
                    if let Some((tile_x, tile_y)) = self.get_tile(&antinode1) {
                        self.grid[tile_x][tile_y].antinodes.push(*k);
                    }

                    let antinode2 = Pos {
                        x: pos2.x - diff.x,
                        y: pos2.y - diff.y,
                    };
                    if let Some((tile_x, tile_y)) = self.get_tile(&antinode2) {
                        self.grid[tile_x][tile_y].antinodes.push(*k);
                    }
                }
            }
        }
    }

    fn get_tile(&self, pos: &Pos) -> Option<(usize, usize)> {
        if pos.x < 0 || pos.y < 0 {
            return None;
        }

        let max_x = self.grid.len() - 1;
        let max_y = self
            .grid
            .first()
            .expect("there should always be at least one row")
            .len()
            - 1;

        let x = pos.x as usize;
        let y = pos.y as usize;
        if x > max_x || y > max_y {
            return None;
        }

        Some((x, y))
    }

    fn count_tiles_with_antinodes(&self) -> i64 {
        self.grid
            .iter()
            .flatten()
            .filter(|t| !t.antinodes.is_empty())
            .count() as i64
    }

    fn render(&self) {
        for row in self.grid.iter() {
            for tile in row.iter() {
                let char = if !tile.antinodes.is_empty() {
                    '#'
                } else if let TileType::Tower(c) = tile.tile_type {
                    c
                } else {
                    '.'
                };
                print!("{char}")
            }
            println!();
        }
    }
}
