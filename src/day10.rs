use std::collections::HashMap;

pub mod puzzle1;
pub mod puzzle2;

type Grid = Vec<Vec<i64>>;

struct Map {
    grid: Grid,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Pos {
    x: i64,
    y: i64,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Map {
    fn parse(content: &str) -> Map {
        let grid: Grid = content
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).expect("should always be a digit") as i64)
                    .collect()
            })
            .collect();

        Map { grid }
    }

    fn get_map_total(&self) -> (i64, i64) {
        let mut score_sum = 0;
        let mut rating_sum = 0;
        for (x, row) in self.grid.iter().enumerate() {
            for (y, val) in row.iter().enumerate() {
                if *val == 0 {
                    let (sum, visited_nines) = self.get_trailhead_score(Pos {
                        x: x as i64,
                        y: y as i64,
                    });
                    score_sum += sum;
                    rating_sum += visited_nines.values().sum::<i64>();
                }
            }
        }
        (score_sum, rating_sum)
    }

    fn get_value(&self, pos: &Pos) -> i64 {
        if let Some(row) = self.grid.get(pos.x as usize) {
            if let Some(value) = row.get(pos.y as usize) {
                return *value;
            }
        }

        -1
    }

    fn get_trailhead_score(&self, starting_pos: Pos) -> (i64, HashMap<Pos, i64>) {
        let mut visited_nines = HashMap::new();
        self.recurse_path(&starting_pos, &mut visited_nines);
        (visited_nines.len() as i64, visited_nines)
    }

    fn recurse_path(&self, current_pos: &Pos, visited_nines: &mut HashMap<Pos, i64>) {
        let current_value = self.get_value(current_pos);

        let directions = [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        for direction in directions {
            let next_pos = match direction {
                Direction::Up => Pos {
                    x: current_pos.x - 1,
                    y: current_pos.y,
                },
                Direction::Down => Pos {
                    x: current_pos.x + 1,
                    y: current_pos.y,
                },
                Direction::Left => Pos {
                    x: current_pos.x,
                    y: current_pos.y - 1,
                },
                Direction::Right => Pos {
                    x: current_pos.x,
                    y: current_pos.y + 1,
                },
            };

            let next_value = self.get_value(&next_pos);
            if next_value == -1 {
                continue;
            }

            if next_value <= current_value {
                continue;
            }

            if next_value - current_value > 1 {
                continue;
            }

            if next_value == 9 {
                if let std::collections::hash_map::Entry::Vacant(e) = visited_nines.entry(next_pos)
                {
                    e.insert(1);
                } else {
                    let entry = visited_nines.get_mut(&next_pos).unwrap();
                    *entry += 1;
                }
                continue;
            }

            self.recurse_path(&next_pos, visited_nines);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn pos_should_implement_hash() {
        let pos1 = Pos { x: 1, y: 2 };
        let pos2 = Pos { x: 1, y: 2 };
        let mut hashset: HashSet<Pos> = HashSet::new();

        hashset.insert(pos1);
        hashset.insert(pos2);

        assert_eq!(1, hashset.len());
    }
}
