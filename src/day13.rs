use std::ops::Add;

use crate::read_file;

pub fn solve_puzzle_1() -> i128 {
    get_result(&read_file("day13.txt"), 0)
}

pub fn solve_puzzle_2() -> i128 {
    get_result(&read_file("day13.txt"), 10000000000000)
}

fn get_result(content: &str, additional_target_distance: i64) -> i128 {
    let games = parse_content(content);

    let mut sum: i128 = 0;

    for game in games {
        let result = game
            .find_cheapest_result(additional_target_distance)
            .unwrap_or_default();

        sum += result as i128;
    }

    sum
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

        let n_b = (self.button_a.x * prize.y - self.button_a.y * prize.x)
            / (self.button_b.y * self.button_a.x - self.button_b.x * self.button_a.y);
        let rem_b = (self.button_a.x * prize.y - self.button_a.y * prize.x)
            % (self.button_b.y * self.button_a.x - self.button_b.x * self.button_a.y);
        let n_a = (prize.x - n_b * self.button_b.x) / self.button_a.x;
        let rem_a = (prize.x - n_b * self.button_b.x) % self.button_a.x;

        if rem_b != 0 || rem_a != 0 {
            return None;
        }

        Some((n_a * A_COST) + (n_b * B_COST))
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

    #[test]
    fn file() {
        dotenvy::dotenv().expect("should be able to load .env file!");

        let result1 = solve_puzzle_1();
        let result2 = solve_puzzle_2();

        assert_eq!(38714, result1);
        assert_eq!(74015623345775, result2);
    }
}
