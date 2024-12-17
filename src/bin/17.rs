use core::panic;

use advent_of_code::split_by_empt_line;

advent_of_code::solution!(17);
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct Op {
    opcode: u8,
    operand: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct Machine {
    a: i64,
    b: i64,
    c: i64,

    sp: usize,
    program: Vec<u8>,
}

impl Machine {
    fn parse_machine(input: &str) -> Machine {
        let split_emptyline = split_by_empt_line(input);
        let machine = Machine {
            a: split_emptyline[0][0][12..].parse::<i64>().unwrap(),
            b: split_emptyline[0][1][12..].parse::<i64>().unwrap(),
            c: split_emptyline[0][2][12..].parse::<i64>().unwrap(),
            sp: 0,
            program: split_emptyline[1][0][9..]
                .split(',')
                .map(|x| x.parse::<u8>().unwrap())
                .collect::<Vec<_>>(),
        };
        machine
    }
    fn get_combo_operand(&self, opr: u8) -> i64 {
        if opr <= 3 {
            opr as i64
        } else if opr == 4 {
            self.a
        } else if opr == 5 {
            self.b
        } else if opr == 6 {
            self.c
        } else {
            panic!("invalid program");
        }
    }
    fn step(&mut self) -> Option<u8> {
        let op = Op {
            opcode: self.program[self.sp],
            operand: self.program[self.sp + 1],
        };
        let mut output = None;
        match op.opcode {
            0 => self.a = self.a / (2_i64.pow(self.get_combo_operand(op.operand) as u32)),
            1 => self.b = self.b ^ op.operand as i64,
            2 => self.b = self.get_combo_operand(op.operand) % 8,
            3 => {
                if self.a != 0 {
                    self.sp = op.operand as usize;
                    return None;
                }
            }
            4 => self.b = self.b ^ self.c,
            5 => output = Some((self.get_combo_operand(op.operand) % 8) as u8),
            6 => self.b = self.a / (2_i64.pow(self.get_combo_operand(op.operand) as u32)),
            7 => self.c = self.a / (2_i64.pow(self.get_combo_operand(op.operand) as u32)),
            _ => {
                panic!("invalid program")
            }
        }

        self.sp += 2;
        output
    }

    fn ready(&self) -> bool {
        self.sp < self.program.len() - 1
    }

    fn run(&mut self) -> Option<String> {
        let mut results = String::new();
        while self.ready() {
            self.step().map(|x| {
                if results.len() != 0 {
                    results += ",";
                }

                results += &x.to_string()
            });
        }
        Some(results)
    }
    fn run_with_goal(&mut self, goal: &Vec<u8>) -> bool {
        let mut results = vec![];
        while self.ready() {
            self.step().map(|x| {
                results.push(x);
            });

            if results.len() > goal.len() {
                return false;
            }

            let same_length = goal
                .iter()
                .zip(results.iter())
                .filter(|(a, b)| a == b)
                .count();
            if same_length < results.len() {
                return false;
            }
            if same_length == goal.len() {
                return true;
            }
        }
        false
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut machine = Machine::parse_machine(input);

    machine.run()
}

pub fn part_two(input: &str) -> Option<i64> {
    let machine = Machine::parse_machine(input);
    let split_emptyline = split_by_empt_line(input);
    let goal_machine = split_emptyline[1][0][9..]
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    let mut a_options = vec![0];
    for i in 0..goal_machine.len() {
        let local_goal = goal_machine
            .iter()
            .skip(goal_machine.len() - i - 1)
            .map(|x| *x)
            .collect::<Vec<u8>>();
        let mut new_a_options = vec![];
        for a in a_options {
            let aa = a << 3;
            for da in 0..8 {
                let mut m = machine.clone();
                m.a = aa + da;
                if m.run_with_goal(&local_goal) {
                    new_a_options.push(aa + da);
                }
            }
        }
        a_options = new_a_options;
    }
    Some(a_options[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("4,6,3,5,6,3,5,2,1,0")));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(String::from("5,7,3,0")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
