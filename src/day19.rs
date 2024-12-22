use std::collections::HashMap;

use crate::read_file;

pub fn solve_puzzle_1() -> i64 {
    let content = read_file("day19.txt");
    get_result(&content)
}

pub fn solve_puzzle_2() -> i64 {
    let content = read_file("day19.txt");
    get_result2(&content)
}

fn get_result(content: &str) -> i64 {
    let puzzle_input = parse_content(content);
    let mut count = 0;
    for design in puzzle_input.designs.iter() {
        if is_design_possible(design, &puzzle_input.patterns) > 0 {
            count += 1;
        }
    }
    count
}

fn get_result2(content: &str) -> i64 {
    let puzzle_input = parse_content(content);
    let mut count = 0;
    for design in puzzle_input.designs.iter() {
        let new_result = is_design_possible(design, &puzzle_input.patterns);
        count += new_result;
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

fn is_design_possible(design: &str, patterns: &[String]) -> i64 {
    let mut contained_patterns: Vec<String> = patterns
        .iter()
        .filter(|p| design.contains(p.as_str()))
        .cloned()
        .collect();
    contained_patterns.sort_by_key(|b| std::cmp::Reverse(b.len()));

    let mut matches = 0;
    for (i, _) in contained_patterns.iter().enumerate() {
        matches += recurse(design, 0, &contained_patterns, i, "", &mut HashMap::new());
    }
    matches
}

fn recurse(
    design: &str,
    cur_design_index: usize,
    patterns: &[String],
    cur_pattern_index: usize,
    cur_value: &str,
    cache: &mut HashMap<String, i64>,
) -> i64 {
    if cur_pattern_index > patterns.len() - 1 {
        return 0;
    }

    let new_pattern = patterns[cur_pattern_index].as_str();
    let rest_of_design = &design[cur_design_index..design.len()];
    if !rest_of_design.starts_with(new_pattern) {
        return 0;
    }

    let next_value = cur_value.to_string() + new_pattern;
    if next_value == design {
        return 1;
    }

    if cache.contains_key(&next_value) {
        return cache[&next_value];
    }

    let mut match_count = 0;
    for (i, _) in patterns.iter().enumerate() {
        let new_design_index = cur_design_index + new_pattern.len();
        match_count += recurse(
            design,
            new_design_index,
            patterns,
            i,
            next_value.as_str(),
            cache,
        );
    }

    cache.insert(next_value, match_count);
    match_count
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
    fn demo2() {
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

        let result = get_result2(content);

        assert_eq!(16, result);
    }

    #[test]
    fn file() {
        dotenvy::dotenv().expect("should be able to load .env file!");

        let result = solve_puzzle_1();
        let result2 = solve_puzzle_2();

        assert_eq!(358, result);
        assert_eq!(600639829400603, result2);
    }
}
