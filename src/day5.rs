use std::collections::{HashMap, HashSet};

pub mod puzzle1;
pub mod puzzle2;

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
