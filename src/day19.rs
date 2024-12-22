use std::collections::VecDeque;

use crate::read_file;

pub fn solve_puzzle_1() -> i64 {
    let content = read_file("day19.txt");
    get_result(&content)
}

fn get_result(content: &str) -> i64 {
    let puzzle_input = parse_content(content);
    let mut count = 0;
    for design in puzzle_input.designs.iter() {
        if is_design_possible(design, &puzzle_input.patterns) {
            count += 1;
        }
    }
    count
}

fn parse_content(content: &str) -> PuzzleInput {
    let mut patterns = Vec::new();
    let mut designs = Vec::new();

    let mut is_designs_section = false;
    for line in content.lines() {
        if line.is_empty() {
            is_designs_section = true;
            continue;
        }

        if !is_designs_section {
            let splits: Vec<&str> = line.split(",").collect();
            for split in splits {
                patterns.push(split.trim().to_string());
            }
        } else {
            designs.push(line.trim().to_string());
        }
    }
    PuzzleInput { patterns, designs }
}

fn is_design_possible(design: &str, patterns: &[String]) -> bool {
    let mut contained_patterns: Vec<&str> = patterns
        .iter()
        .filter(|p| design.contains(p.as_str()))
        .map(|p| p.as_str())
        .collect();
    contained_patterns.sort_by_key(|b| std::cmp::Reverse(b.len()));

    let mut patterns_to_check: VecDeque<String> = VecDeque::new();
    for starts_with in contained_patterns.iter().filter(|p| design.starts_with(*p)) {
        patterns_to_check.push_front(starts_with.to_string());
    }

    while let Some(pattern_match) = patterns_to_check.pop_front() {
        if pattern_match == design {
            return true;
        }
        let rest_of_design = &design[pattern_match.len()..design.len()];
        for p in contained_patterns.iter() {
            if rest_of_design.starts_with(p) {
                let mut new = pattern_match.clone();
                new.push_str(p);
                patterns_to_check.push_front(new);
            }
        }
    }

    false
}

#[derive(Debug)]
struct PuzzleInput {
    patterns: Vec<String>,
    designs: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let content = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

        let result = get_result(content);

        assert_eq!(6, result);
    }

    #[test]
    fn file() {
        dotenvy::dotenv().expect("should be able to load .env file!");

        let result = solve_puzzle_1();

        assert_eq!(358, result);
    }
}
