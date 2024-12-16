use std::ops::{Add, Mul};

use crate::read_file;

pub fn solve_puzzle_1() -> i64 {
    let content = read_file("day14.txt");
    let mut grid = Grid::parse(&content, 101, 103);
    get_result(&mut grid, 100)
}

fn get_result(grid: &mut Grid, ticks: i64) -> i64 {
    grid.render();
    grid.run(ticks);
    grid.solve()
}

#[derive(Debug, Copy, Clone)]
struct Pos {
    x: i64,
    y: i64,
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul for Pos {
    type Output = Pos;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Pos {
    fn parse(line: &str) -> Self {
        let splits: Vec<&str> = line.split(',').collect();
        let x = splits
            .first()
            .expect("should always be two parts")
            .parse()
            .expect("should always be a number");
        let y = splits
            .get(1)
            .expect("should always be two parts")
            .parse()
            .expect("should always be a number");

        Self { x, y }
    }
}

#[derive(Debug)]
struct Robot {
    pos: Pos,
    velocity: Pos,
}

impl Robot {
    fn parse(line: &str) -> Self {
        let splits: Vec<&str> = line.split_whitespace().collect();
        let pos = splits.first().expect("should always have two parts");
        let velocity = splits.get(1).expect("should always have two parts");

        Self {
            pos: Pos::parse(&pos.replace("p=", "")),
            velocity: Pos::parse(&velocity.replace("v=", "")),
        }
    }
}

#[derive(Debug)]
struct Grid {
    max_x: i64,
    max_y: i64,
    robots: Vec<Robot>,
}

impl Grid {
    fn parse(content: &str, max_x: i64, max_y: i64) -> Grid {
        let mut robots = Vec::new();
        for line in content.lines() {
            robots.push(Robot::parse(line));
        }

        Self {
            max_x,
            max_y,
            robots,
        }
    }

    fn render(&self) {
        (0..self.max_y).for_each(|y| {
            (0..self.max_x).for_each(|x| {
                let robot_count = self
                    .robots
                    .iter()
                    .filter(|r| r.pos.x == x && r.pos.y == y)
                    .count();

                if robot_count == 0 {
                    print!(".");
                } else {
                    print!("{robot_count}");
                }
            });
            println!();
        });
    }

    fn run(&mut self, ticks: i64) {
        for robot in self.robots.iter_mut() {
            robot.pos = robot.pos + (Pos { x: ticks, y: ticks } * robot.velocity);

            if robot.pos.x < 0 {
                robot.pos.x %= -self.max_x;
                robot.pos.x += self.max_x;
            }
            if robot.pos.x > self.max_x {
                robot.pos.x %= self.max_x;
            }

            if robot.pos.y < 0 {
                robot.pos.y %= -self.max_y;
                robot.pos.y += self.max_y;
            }
            if robot.pos.y > self.max_y {
                robot.pos.y %= self.max_y;
            }

            if robot.pos.x == self.max_x {
                robot.pos.x -= self.max_x;
            }

            if robot.pos.y == self.max_y {
                robot.pos.y -= self.max_y;
            }
        }
    }

    fn solve(&self) -> i64 {
        let x_divider = (self.max_x - 1) / 2;
        let y_divider = (self.max_y - 1) / 2;
        let mut quad1 = 0;
        let mut quad2 = 0;
        let mut quad3 = 0;
        let mut quad4 = 0;

        for robot in self.robots.iter() {
            if robot.pos.x == x_divider {
                continue;
            }

            if robot.pos.y == y_divider {
                continue;
            }

            if robot.pos.x < x_divider && robot.pos.y < y_divider {
                quad1 += 1;
            }

            if robot.pos.x > x_divider && robot.pos.y < y_divider {
                quad2 += 1;
            }

            if robot.pos.x < x_divider && robot.pos.y > y_divider {
                quad3 += 1;
            }

            if robot.pos.x > x_divider && robot.pos.y > y_divider {
                quad4 += 1;
            }
        }

        quad1 * quad2 * quad3 * quad4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let content = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

        let result = get_result(&mut Grid::parse(content, 11, 7), 100);

        assert_eq!(12, result);
    }

    #[test]
    fn file() {
        dotenvy::dotenv().expect("should be able to load .env file!");

        let result = solve_puzzle_1();

        assert_eq!(229868730, result);
    }
}
