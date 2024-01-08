#![allow(dead_code)]

use std::fs::File;

use std::io::BufRead;
use std::io::BufReader;

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

fn compute_next(numbers: &Vec<i64>) -> i64 {
    let mut diff: Vec<i64> = Vec::new();
    diff.resize(numbers.len() - 1, 0);
    let mut all_zeros = true;
    for i in 0..diff.len() {
        diff[i] = numbers[i + 1] - numbers[i];
        if diff[i] != 0 {
            all_zeros = false;
        }
    }

    if all_zeros {
        return numbers[numbers.len() - 1];
    } else {
        let diff_next = compute_next(&diff);
        return numbers[numbers.len() - 1] + diff_next;
    }
}

fn compute_prev(numbers: &Vec<i64>) -> i64 {
    let mut diff: Vec<i64> = Vec::new();
    diff.resize(numbers.len() - 1, 0);
    let mut all_zeros = true;
    for i in 0..diff.len() {
        diff[i] = numbers[i + 1] - numbers[i];
        if diff[i] != 0 {
            all_zeros = false;
        }
    }

    if all_zeros {
        return numbers[0];
    } else {
        let diff_prev = compute_prev(&diff);
        return numbers[0] - diff_prev;
    }
}

fn process_content(content: Vec<String>) {
    let mut res = 0;
    for (_line_index, line) in content.iter().enumerate() {
        // new line, flush
        let numbers: Vec<i64> = line
            .trim()
            .split(' ')
            .map(|x| x.to_string().parse::<i64>().unwrap())
            .collect();
        res += compute_prev(&numbers);
    }
    println!("res {}", res);
}

fn main() {
    // Day 1 first problem
    let _line = String::new();
    // let _i: std::i32 = 50;
    let file_path = "input.txt";
    //let file_path = "input.txt";
    let content = read_file(file_path);
    process_content(content);
}
