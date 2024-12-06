use std::collections::HashMap;

use crate::read_file;

use super::{parse_content, RuleSet};

pub fn solve_day5_puzzle2() -> i64 {
    let content = read_file("day5.txt");
    get_result(content.as_str())
}

pub fn get_result(content: &str) -> i64 {
    let (rules, sections) = parse_content(content);

    let mut sum = 0;

    for section in sections {
        let mut keep_reordering = true;
        let mut section_to_use = section;

        if let Some(s) = reorder_section(&section_to_use, &rules) {
            section_to_use = s;
        } else {
            continue;
        }

        while keep_reordering {
            if let Some(new_section) = reorder_section(&section_to_use, &rules) {
                section_to_use = new_section;
            } else {
                keep_reordering = false;
            }
        }

        let middle_index = section_to_use.len() / 2;
        sum += section_to_use[middle_index];
    }

    sum
}

fn reorder_section(section: &[i64], rules: &HashMap<i64, RuleSet>) -> Option<Vec<i64>> {
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
            let mut new_section: Vec<i64> = Vec::new();
            for (x, v) in section.iter().enumerate() {
                if x == i {
                    new_section.push(section[j]);
                } else if x == j {
                    new_section.push(section[i]);
                } else {
                    new_section.push(*v);
                }
            }

            return Some(new_section);
        }

        i += 1;
    }

    None
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

        assert_eq!(123, result);
    }
}
