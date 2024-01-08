use std::fs::File;

use std::io::BufRead;
use std::io::BufReader;

struct State {
    _accumulator: String,
}

impl State {
    fn new() -> State {
        State {
            _accumulator: String::new(),
        }
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

fn process_content(content: Vec<String>) {
    let mut _numstate = State::new();

    let mut table: Vec<i32> = Vec::new();
    table.resize_with(198, || 1);

    for (_line_index, line) in content.iter().enumerate() {
        // new line, flush
        let mut split = line.split(":");
        let card_id: i32 = (split
            .next()
            .unwrap()
            .to_string()
            .replace("Card", "")
            .trim()
            .parse::<i32>()
            .unwrap())
            - 1;
        println!("Card id {}", card_id);
        let remainder = split.next().unwrap();
        assert_eq!(split.next(), None);

        //
        // println!("{}", remainder);
        let mut split = remainder.split("|");

        let win_numbers: Vec<i32> = split
            .next()
            .unwrap()
            .to_string()
            .trim()
            .split(" ")
            .filter_map(|x| x.to_string().parse().ok())
            .collect();
        let my_numbers: Vec<i32> = split
            .next()
            .unwrap()
            .to_string()
            .trim()
            .split(" ")
            .filter_map(|x| x.to_string().parse().ok())
            .collect();
        assert_eq!(split.next(), None);

        //
        let mut matching_numbers = 0;
        for num in my_numbers {
            if win_numbers.contains(&num) {
                matching_numbers += 1;
            }
        }

        for i in 1..matching_numbers + 1 {
            let idx: usize = (card_id + i) as usize;
            table[idx] += table[card_id as usize];
        }
    }

    let mut res = 0;
    for i in table.iter() {
        res += i;
    }
    println!("Final sum {}", res);
}

fn main() {
    let _line = String::new();
    let file_path = "input.txt";
    let content = read_file(file_path);
    process_content(content);
}
