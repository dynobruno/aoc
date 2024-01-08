use std::fs::File;

use std::io::BufRead;
use std::io::BufReader;

struct State {
    red: i32,
    blue: i32,
    green: i32,
}

impl State {
    fn set_max(&mut self, color: String, count: i32) {
        if color == "red" {
            self.red = std::cmp::max(self.red, count);
        };
        if color == "green" {
            self.green = std::cmp::max(self.green, count);
        };
        if color == "blue" {
            self.blue = std::cmp::max(self.blue, count);
        };
    }
}

fn process_line(line: String) -> i32 {
    let mut res = State {
        red: 0,
        blue: 0,
        green: 0,
    };
    let mut split = line.split(":");
    let _game_id: i32 = split
        .next()
        .unwrap()
        .to_string()
        .replace("Game ", "")
        .parse()
        .unwrap();
    let remainder = split.next().unwrap();
    assert_eq!(split.next(), None);
    let mut split = remainder.split(";");
    while let Some(x) = split.next() {
        let ss = x.split(',');
        let rr: Vec<String> = ss.map(|x| x.trim().to_string()).collect();
        for item in rr.into_iter() {
            let mut bb = item.split(' ');
            let number: i32 = bb.next().unwrap().to_string().parse().unwrap();
            let color: String = bb.next().unwrap().to_string();
            res.set_max(color, number);
        }
    }

    // if (res.blue <= 14) && (res.red <= 12) && (res.green <= 13) {
    //     game_id
    // } else {
    //     0
    // }
    res.blue * res.red * res.green
}

fn process(filepath: &str) -> i32 {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut res: i32 = 0;

    // run
    for line in reader.lines() {
        match line {
            Ok(a) => {
                res += process_line(a);
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
    // let file_path = "input_test.txt";
    let file_path = "input.txt";
    let r = process(file_path);
    println!("{}", r);
}

#[cfg(test)]
mod tests {
    use crate::process_line;

    #[test]
    fn test_lines() {
        assert_eq!(process_line("seven6fourtwotwo".to_string()), 0);
    }
}
