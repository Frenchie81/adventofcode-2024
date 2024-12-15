use core::f64;
use std::{ops::Add, sync::mpsc::channel};

use rayon::prelude::*;

use crate::read_file;

pub fn solve_puzzle_1() -> i64 {
    get_result(&read_file("day13.txt"), 0)
}

pub fn solve_puzzle_2() -> i64 {
    get_result(&read_file("day13.txt"), 10000000000000)
}
fn get_result(content: &str, additional_target_distance: i64) -> i64 {
    let games = parse_content(content);

    let (s, r) = channel();

    games.into_par_iter().for_each_with(s, |s, g| {
        println!("starting: {:?}", g.prize);
        let r = g
            .find_cheapest_result(additional_target_distance)
            .unwrap_or_default();
        println!("result: {r}");
        s.send(r);
    });

    let mut res: Vec<i64> = r.iter().collect();

    // for game in games {
    //     result += game
    //         .find_cheapest_result(additional_target_distance)
    //         .unwrap_or_default();
    // }
    res.iter().sum()
}

fn parse_content(content: &str) -> Vec<Game> {
    let mut games = vec![];
    let mut game_string = String::new();

    let mut init = true;
    for line in content.lines().filter(|l| !l.is_empty()) {
        if !init && line.starts_with("Button A") {
            games.push(Game::parse(&game_string));
            game_string = String::new()
        }
        init = false;

        game_string.push_str(line);
        game_string.push('\n');
    }

    // add the last game
    games.push(Game::parse(&game_string));

    games
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug)]
struct Game {
    button_a: Pos,
    button_b: Pos,
    prize: Pos,
}

impl Game {
    fn parse(value: &str) -> Self {
        let lines: Vec<&str> = value.lines().collect();
        let button_a_splits: Vec<&str> = lines[0].split(':').last().unwrap().split(',').collect();
        let button_b_splits: Vec<&str> = lines[1].split(':').last().unwrap().split(',').collect();
        let prize_splits: Vec<&str> = lines[2].split(':').last().unwrap().split(',').collect();

        let button_a = Pos::new(
            button_a_splits[0].replace("X+", "").trim().parse().unwrap(),
            button_a_splits[1].replace("Y+", "").trim().parse().unwrap(),
        );
        let button_b = Pos::new(
            button_b_splits[0].replace("X+", "").trim().parse().unwrap(),
            button_b_splits[1].replace("Y+", "").trim().parse().unwrap(),
        );
        let prize = Pos::new(
            prize_splits[0].replace("X=", "").trim().parse().unwrap(),
            prize_splits[1].replace("Y=", "").trim().parse().unwrap(),
        );

        Self {
            button_a,
            button_b,
            prize,
        }
    }

    fn find_cheapest_result(&self, additional_target_distance: i64) -> Option<i64> {
        const A_COST: i64 = 3;
        const B_COST: i64 = 1;

        let prize = self.prize
            + Pos {
                x: additional_target_distance,
                y: additional_target_distance,
            };

        let max_b_x_presses = prize.x / self.button_b.x;
        let max_b_y_presses = prize.y / self.button_b.y;

        let max_b_presses = if max_b_y_presses > max_b_x_presses {
            max_b_y_presses
        } else {
            max_b_x_presses
        };

        for b_presses in (1..max_b_presses).rev() {
            let b_val = self.button_b.x * b_presses;
            let a_val = (prize.x - b_val) as f64 / self.button_a.x as f64;
            if a_val.fract() == 0.0 {
                let answer_x = (self.button_a.x * a_val as i64) + (self.button_b.x * b_presses);
                let answer_y = (self.button_a.y * a_val as i64) + (self.button_b.y * b_presses);
                if answer_x == prize.x && answer_y == prize.y {
                    return Some(b_presses * B_COST + a_val as i64 * A_COST);
                }
            }
        }

        // a*94 + b*22 = 8400

        // let b_table = ButtonPossibility {
        //     iteration: 0,
        //     base_pos: self.button_b,
        //     max_pos: prize,
        //     modifier: 10000000000000,
        // };
        //
        // for (b, b_press) in b_table.enumerate() {
        //     let a_table = ButtonPossibility {
        //         iteration: 0,
        //         base_pos: self.button_a,
        //         max_pos: prize,
        //         modifier: 10000000000000,
        //     };
        //
        //     for (a, a_press) in a_table.enumerate() {
        //         if a_press + b_press == prize {
        //             let a_total = (a as i64) * A_COST;
        //             let b_total = (b as i64) * B_COST;
        //
        //             return Some(a_total + b_total);
        //         }
        //     }
        // }

        None
    }
}

#[derive(Copy, Clone)]
struct ButtonPossibility {
    iteration: i64,
    base_pos: Pos,
    max_pos: Pos,
    modifier: i64,
}

impl Iterator for ButtonPossibility {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let next = Pos {
            x: self.base_pos.x * self.iteration,
            y: self.base_pos.y * self.iteration,
        };
        if next.x > self.max_pos.x && next.y > self.max_pos.y {
            None
        } else {
            self.iteration += self.modifier;
            Some(next)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let content = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

        let result = get_result(content, 0);

        assert_eq!(480, result);
    }
}
