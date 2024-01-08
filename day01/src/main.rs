use std::fs::File;
// use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;

struct Machine {
    expected: String,
    real_value: Option<i32>,
    //
    accumulator: String,
}

impl Machine {
    fn new(expected: String, real_value: Option<i32>) -> Machine {
        Machine {
            expected: expected,
            accumulator: String::new(),
            real_value: real_value,
        }
    }

    fn feed(&mut self, c: char) -> Option<i32> {
        self.accumulator.push(c);
        if self.accumulator.ends_with(&self.expected) {
            match self.real_value {
                Some(x) => Some(x),
                None => Some(self.expected.parse::<i32>().unwrap()),
            }
        } else {
            None
        }
    }

    fn reset(&mut self) {
        self.accumulator.clear();
    }
}

fn get_machines() -> Vec<Machine> {
    let mut read_machines: Vec<Machine> = Vec::new();
    read_machines.push(Machine::new("0".to_string(), None));
    read_machines.push(Machine::new("1".to_string(), None));
    read_machines.push(Machine::new("2".to_string(), None));
    read_machines.push(Machine::new("3".to_string(), None));
    read_machines.push(Machine::new("4".to_string(), None));
    read_machines.push(Machine::new("5".to_string(), None));
    read_machines.push(Machine::new("6".to_string(), None));
    read_machines.push(Machine::new("7".to_string(), None));
    read_machines.push(Machine::new("8".to_string(), None));
    read_machines.push(Machine::new("9".to_string(), None));
    read_machines.push(Machine::new("one".to_string(), Some(1)));
    read_machines.push(Machine::new("two".to_string(), Some(2)));
    read_machines.push(Machine::new("three".to_string(), Some(3)));
    read_machines.push(Machine::new("four".to_string(), Some(4)));
    read_machines.push(Machine::new("five".to_string(), Some(5)));
    read_machines.push(Machine::new("six".to_string(), Some(6)));
    read_machines.push(Machine::new("seven".to_string(), Some(7)));
    read_machines.push(Machine::new("eight".to_string(), Some(8)));
    read_machines.push(Machine::new("nine".to_string(), Some(9)));
    read_machines
}

fn process_line(read_machines: &mut Vec<Machine>, line: String) -> i32 {
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;
    for machine in read_machines.iter_mut() {
        machine.reset();
    }
    for c in line.chars() {
        for machine in read_machines.iter_mut() {
            let ret = machine.feed(c);
            match ret {
                Some(x) => {
                    let ic = Some(x.to_string().chars().nth(0).unwrap());
                    if first.is_none() {
                        first = ic;
                    }
                    last = ic;
                }
                None => {}
            }
        }
    }

    let mut strres: String = String::new();
    strres += first.unwrap().to_string().as_str();
    strres += last.unwrap().to_string().as_str();
    let res = strres.parse::<i32>().unwrap();
    println!("process line {} {} {}", strres, res, line);
    res
}

fn day1_pb1(filepath: &str) -> i32 {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut res: i32 = 0;

    // initialize reading machine
    let mut read_machines: Vec<Machine> = get_machines();

    // run
    for line in reader.lines() {
        match line {
            Ok(a) => {
                res += process_line(&mut read_machines, a);
            }
            Err(_) => {}
        }
    }

    return res;
}

fn main() {
    // Day 1 first problem
    let _line = String::new();
    // let _i: std::i32 = 50;
    let file_path = "/home/bruno/workspace/advent_of_code/day01/input1.txt";
    let r = day1_pb1(file_path);
    println!("{}", r);
}

#[cfg(test)]
mod tests {
    use crate::{get_machines, process_line, Machine};

    #[test]
    fn test_lines() {
        let mut read_machines: Vec<Machine> = get_machines();
        assert_eq!(
            process_line(&mut read_machines, "seven6fourtwotwo".to_string()),
            72
        );

        assert_eq!(
            process_line(
                &mut read_machines,
                "eightcvbzqczt9ninegxlpsevenfour".to_string()
            ),
            84
        );

        assert_eq!(
            process_line(&mut read_machines, "threeoneeight758threegtwo".to_string()),
            32
        );

        let v = [
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
            ("1spnthree59ninejjgjdlx", 19),
            ("thffrjkhsixsix1one7one", 61),
            ("sevengseven71nklkzdeightsevenfive", 75),
            ("76eighttwoqzfnllqnkq", 72),
            ("3twoone", 31),
        ];

        for (a, b) in v.iter() {
            assert_eq!(process_line(&mut read_machines, a.to_string()), *b);
        }
    }
}
