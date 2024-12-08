pub mod puzzle1;
pub mod puzzle2;

#[derive(Debug, Clone)]
struct Equation {
    test_value: i64,
    values: Vec<i64>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operation {
    Add(i64),
    Multiply(i64),
}

impl Operation {
    fn next(&self) -> Self {
        match self {
            Operation::Add(i) => Operation::Multiply(*i),
            Operation::Multiply(i) => Operation::Add(*i),
        }
    }
}

impl Equation {
    fn parse(line: &str) -> Self {
        let colon_split: Vec<&str> = line.split(':').collect();
        let left = colon_split.first().expect("there should always be a colon");
        let right = colon_split.get(1).expect("there should always be a colon");

        let test_value: i64 = left
            .trim()
            .parse()
            .expect("left should always be an integer");

        let values: Vec<i64> = right
            .split_whitespace()
            .map(|x| x.trim().parse().expect("values should all be int"))
            .collect();

        Self { test_value, values }
    }

    fn can_solve(&self) -> bool {
        let values_len = self.values.len();
        let value = self
            .values
            .first()
            .expect("there should always be at least one value");
        if values_len == 1 {
            return *value == self.test_value;
        }

        let mut operations: Vec<Operation> = Vec::new();
        for x in 1..values_len {
            operations.push(Operation::Add(self.values[x]));
        }

        loop {
            let mut answer = *value;
            for o in operations.iter() {
                answer = match o {
                    Operation::Add(i) => answer + i,
                    Operation::Multiply(i) => answer * i,
                }
            }

            if answer == self.test_value {
                return true;
            }

            if all_multiply(&operations) {
                break;
            }

            let mut value_added = false;
            for x in 0..operations.len() {
                match operations[x] {
                    Operation::Add(_) => {
                        operations[x] = operations[x].next();
                        value_added = true;
                    }
                    Operation::Multiply(_) => operations[x] = operations[x].next(),
                }

                if value_added {
                    break;
                }
            }
        }
        false
    }
}

fn parse_contents(content: &str) -> Vec<Equation> {
    content.lines().map(Equation::parse).collect()
}

fn all_multiply(operations: &[Operation]) -> bool {
    for o in operations.iter() {
        match o {
            Operation::Add(_) => return false,
            Operation::Multiply(_) => (),
        }
    }

    true
}
