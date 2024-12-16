use std::ops::{Add, Mul};

use crate::read_file;

pub fn solve_puzzle_1() -> i64 {
    let content = read_file("day14.txt");
    let mut grid = Grid::parse(&content, 101, 103);
    get_result(&mut grid)
}

fn get_result(grid: &mut Grid) -> i64 {
    grid.render();
    grid.run(100);
    println!();
    grid.render();
    0
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
        let y = splits
            .first()
            .expect("should always be two parts")
            .parse()
            .expect("should always be a number");
        let x = splits
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
        (0..self.max_y).for_each(|x| {
            (0..self.max_x).for_each(|y| {
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
                robot.pos.x = robot.pos.x.abs();
            }

            if robot.pos.y < 0 {
                robot.pos.y = robot.pos.y.abs();
            }

            if robot.pos.x > self.max_x {
                robot.pos.x %= self.max_x;
            }

            if robot.pos.y > self.max_y {
                robot.pos.y %= self.max_y;
            }
        }
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

        let result = get_result(&mut Grid::parse(content, 11, 7));

        assert_eq!(12, result);
    }

    #[test]
    fn temp() {
        let robot = Robot {
            pos: Pos { x: 0, y: 0 },
            velocity: Pos { x: 2, y: 1 },
        };

        let ticks = 3;
        let next_x_pos = robot.pos.x + (robot.velocity.x * ticks);
        let next_y_pos = robot.pos.y + (robot.velocity.y * ticks);

        let new_pos = Pos {
            x: next_x_pos,
            y: next_y_pos,
        };

        println!("{new_pos:?}");
    }
}
