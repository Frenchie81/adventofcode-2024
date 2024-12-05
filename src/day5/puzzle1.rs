use core::panic;
use std::collections::{HashMap, HashSet};

use crate::read_file;

pub fn solve_day5_puzzle1() -> i64 {
    let content = read_file("day5.txt");
    get_result(content.as_str())
}

fn get_result(content: &str) -> i64 {
    let (rules, sections) = parse_content(content);

    let mut sum = 0;

    for section in sections {
        let section_len = section.len();
        let mut i = 0;
        let mut is_valid = true;

        while i < section_len {
            let value = section[i];
            let mut j = i + 1;

            while j < section_len {
                let next_value = section[j];
                if rules.contains_key(&next_value) {
                    let rule = &rules[&next_value];
                    if rule.is_valid_after(&value) {
                        is_valid = false;
                        break;
                    }
                }

                j += 1;
            }

            if !is_valid {
                break;
            }

            i += 1;
        }

        if is_valid {
            let middle_index = section_len / 2;
            let middle = &section[middle_index];
            sum += middle;
        }
    }

    sum
}

struct RuleSet {
    before: HashSet<i64>,
}

impl RuleSet {
    fn new() -> RuleSet {
        RuleSet {
            before: HashSet::new(),
        }
    }

    fn add_rule(&mut self, before: i64) {
        self.before.insert(before);
    }

    fn is_valid_after(&self, value: &i64) -> bool {
        self.before.contains(value)
    }
}

fn parse_content(content: &str) -> (HashMap<i64, RuleSet>, Vec<Vec<i64>>) {
    let mut rules: HashMap<i64, RuleSet> = HashMap::new();
    let mut sections: Vec<Vec<i64>> = Vec::new();

    let mut parse_rules = true;

    for line in content.lines() {
        if line.is_empty() {
            parse_rules = false;
            continue;
        }

        if parse_rules {
            let splits: Vec<&str> = line.split('|').collect();
            if splits.len() != 2 {
                panic!("rule {line} could not be parsed!");
            }

            let key: i64 = splits[0].parse().expect("rule key must be a number");
            let before: i64 = splits[1].parse().expect("rule value must be a number");
            let rule = rules.entry(key).or_insert_with(RuleSet::new);

            rule.add_rule(before);
        } else {
            let splits: Vec<i64> = line
                .split(',')
                .map(|v| v.trim().parse::<i64>().unwrap())
                .collect();

            sections.push(splits);
        }
    }

    (rules, sections)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let content = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let result = get_result(content);

        assert_eq!(143, result);
    }
}
