use crate::read_file;

pub fn solve_puzzle_1() -> String {
    let content = read_file("day17.txt");
    get_result(&content)
}

pub fn solve_puzzle_2() -> i64 {
    let content = read_file("day17.txt");
    get_result2(&content)
}

fn get_result(content: &str) -> String {
    let (mut computer, program) = parse_content(content);
    let result: Vec<String> = computer
        .execute(&program)
        .iter()
        .map(|i| i.to_string())
        .collect();
    result.join(",")
}

fn get_result2(content: &str) -> i64 {
    let (computer, program) = parse_content(content);
    let mut output: Vec<i64> = Vec::new();
    let mut reg_a = 0;
    while output != program {
        reg_a += 1;
        let mut copy = computer;
        copy.a = reg_a;
        output = copy.execute(&program);
    }

    reg_a
}

fn parse_content(content: &str) -> (Computer, Vec<i64>) {
    let lines: Vec<&str> = content.lines().collect();
    let line0 = lines.first().unwrap();
    let line1 = lines.get(1).unwrap();
    let line2 = lines.get(2).unwrap();
    let line4 = lines.get(4).unwrap();

    let reg_1 = line0.split(':').collect::<Vec<&str>>();
    let reg_1: i64 = reg_1.get(1).unwrap().trim().parse().unwrap();

    let reg_2 = line1.split(':').collect::<Vec<&str>>();
    let reg_2: i64 = reg_2.get(1).unwrap().trim().parse().unwrap();

    let reg_3 = line2.split(':').collect::<Vec<&str>>();
    let reg_3: i64 = reg_3.get(1).unwrap().trim().parse().unwrap();

    let program = line4.split(':').collect::<Vec<&str>>();
    let program: Vec<i64> = program
        .get(1)
        .unwrap()
        .split(',')
        .map(|v| v.trim().parse().unwrap())
        .collect();

    let computer = Computer {
        a: reg_1,
        b: reg_2,
        c: reg_3,
    };

    (computer, program)
}

#[derive(Debug, Copy, Clone)]
struct Computer {
    a: i64,
    b: i64,
    c: i64,
}

impl Computer {
    fn execute(&mut self, instructions: &Vec<i64>) -> Vec<i64> {
        let mut output = Vec::new();
        let mut p = 0; // instruction pointer
        while p < instructions.len() {
            let mut increment_pointer = true;
            if let Some(opcode) = instructions.get(p) {
                if let Some(operand) = instructions.get(p + 1) {
                    match opcode {
                        0 => {
                            // adv
                            let val1 = self.a;
                            let combo = self.get_combo_value(*operand);
                            self.a = val1 / 2_i64.pow(combo as u32);
                        }
                        1 => {
                            // bxl
                            self.b ^= operand;
                        }
                        2 => {
                            // bst
                            let combo = self.get_combo_value(*operand);
                            self.b = combo % 8;
                        }
                        3 => {
                            // jnz
                            if self.a > 0 {
                                p = *operand as usize;
                                increment_pointer = false;
                            }
                        }
                        4 => {
                            // bxc
                            self.b ^= self.c;
                        }
                        5 => {
                            // out
                            let out = self.get_combo_value(*operand) % 8;
                            output.push(out);
                        }
                        6 => {
                            // bdv
                            let val1 = self.a;
                            let combo = self.get_combo_value(*operand);
                            self.b = val1 / 2i64.pow(combo as u32);
                        }
                        7 => {
                            // cdv
                            let val1 = self.a;
                            let combo = self.get_combo_value(*operand);
                            self.c = val1 / 2i64.pow(combo as u32);
                        }
                        x => panic!("unknown instruction {x}"),
                    }
                }
            }
            if increment_pointer {
                p += 2;
            }
        }
        output
    }

    fn get_combo_value(&self, combo: i64) -> i64 {
        if combo > 0 && combo < 4 {
            combo
        } else if combo == 4 {
            self.a
        } else if combo == 5 {
            self.b
        } else if combo == 6 {
            self.c
        } else {
            panic!("bad combo specified!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let content = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

        let result = get_result(content);

        assert_eq!("4,6,3,5,6,3,5,2,1,0", result)
    }

    #[test]
    fn demo2() {
        let content = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

        let result = get_result2(content);

        assert_eq!(117440, result)
    }

    #[test]
    fn file() {
        dotenvy::dotenv().expect("should be able to load .env file!");

        let result = solve_puzzle_1();

        assert_eq!("2,3,4,7,5,7,3,0,7", result);
    }
}
