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

fn process_content(content: Vec<String>) {
    for (line_index, line) in content.iter().enumerate() {
        // new line, flush
    }
}

fn main() {
    // Day 1 first problem
    let _line = String::new();
    // let _i: std::i32 = 50;
    let file_path = "test_input.txt";
    //let file_path = "input.txt";
    let content = read_file(file_path);
    process_content(content);
}
