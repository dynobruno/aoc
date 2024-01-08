use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;

use std::io::BufRead;
use std::io::BufReader;

struct State {
    accumulator: String,
    col_start: usize,
    col_end: usize,
    line: usize,
}

struct Number {
    col_start: usize,
    col_end: usize,
    line: usize,
    value: usize,
}

impl Number {
    fn is_near(&self, line_index: usize, col_index: usize) -> bool {
        let mut cond: bool = true;
        cond &= (line_index + 1) >= self.line;
        cond &= line_index <= (self.line + 1);
        cond &= (col_index + 1) >= self.col_start;
        cond &= col_index <= (self.col_end + 1);
        cond
    }
}

impl State {
    fn new() -> State {
        State {
            accumulator: String::new(),
            col_start: 0,
            col_end: 0,
            line: 0,
        }
    }
    fn push(&mut self, line: usize, column: usize, c: char) {
        self.line = line;
        if self.accumulator.is_empty() {
            self.col_start = column
        }
        self.col_end = column;
        self.accumulator.push(c);
    }
    fn get_number(&self) -> Option<usize> {
        if self.accumulator.len() > 0 {
            Some(self.accumulator.parse().unwrap())
        } else {
            None
        }
    }
    fn flush(&mut self) -> Option<Number> {
        let res = match self.get_number() {
            Some(number) => Some(Number {
                col_start: self.col_start,
                col_end: self.col_end,
                line: self.line,
                value: number,
            }),
            None => None,
        };

        self.accumulator.clear();
        self.col_start = 0;
        self.col_end = 0;

        res
    }
}

fn read_file(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut res: Vec<String> = Vec::new();
    // run
    for line in reader.lines() {
        match line {
            Ok(a) => res.push(a),
            Err(_) => {}
        }
    }

    return res;
}

struct Symbols {
    data: HashMap<usize, HashSet<usize>>,
}

impl Symbols {
    fn new() -> Symbols {
        Symbols {
            data: HashMap::new(),
        }
    }

    fn push(&mut self, line_index: usize, col_index: usize) {
        if !self.data.contains_key(&line_index) {
            let mut hh = HashSet::new();
            hh.insert(col_index);
            self.data.insert(line_index, hh);
        }
        let cols = self.data.get_mut(&line_index).unwrap();
        cols.insert(col_index);
    }

    fn contains(&self, line_index: usize, col_index: usize) -> bool {
        if let Some(colmap) = self.data.get(&line_index) {
            colmap.contains(&col_index)
        } else {
            false
        }
    }
}

fn process_content(content: Vec<String>) {
    let mut numstate = State::new();
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols = Symbols::new();
    let mut star_symbols = Symbols::new();

    for (line_index, line) in content.iter().enumerate() {
        // new line, flush
        match numstate.flush() {
            Some(x) => numbers.push(x),
            _ => {}
        }
        for (col_index, char) in line.chars().enumerate() {
            if char.is_numeric() {
                numstate.push(line_index as usize, col_index as usize, char);
            } else {
                // different character, flush eventually
                match numstate.flush() {
                    Some(x) => numbers.push(x),
                    _ => {}
                }

                if char == '.' {
                    // nothing to do
                } else if char == '*' {
                    star_symbols.push(line_index, col_index);
                } else {
                    // symbols
                    symbols.push(line_index, col_index);
                }
            }
        }
    }

    let mut res = 0;

    for number in numbers.iter() {
        let mut found = false;
        let rs: i32 = number.col_start as i32 - 1;
        let re: i32 = number.col_end as i32 + 1;
        for i in rs..re + 1 {
            if i < 0 {
                continue;
            }
            for li in -1..1 + 1 {
                let lii: i32 = number.line as i32 + li;
                if lii >= 0 {
                    if symbols.contains(lii as usize, i as usize) {
                        found = true;
                    }
                }
            }
        }
        if found {
            res += number.value;
        }
    }

    let mut res2 = 0;

    for (line_index, col_list) in star_symbols.data.iter() {
        for col_index in col_list.iter() {
            let mut near_numbers: Vec<&Number> = Vec::new();
            for numb in numbers.iter() {
                if numb.is_near(*line_index, *col_index) {
                    near_numbers.push(&numb);
                }
            }
            if near_numbers.len() == 2 {
                let gear_ratio = near_numbers[0].value * near_numbers[1].value;
                res2 += gear_ratio;
            }
        }
    }

    println!("Final sum {}", res);
    println!("Final sum gear ratio {}", res2);
}

fn main() {
    // Day 1 first problem
    let _line = String::new();
    // let _i: std::i32 = 50;
    // let file_path = "input_test.txt";
    let file_path = "input.txt";
    let content = read_file(file_path);
    process_content(content);
}
