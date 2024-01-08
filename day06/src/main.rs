use std::collections::VecDeque;
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
    let mut pb_times: Vec<i64> = Vec::new();
    let mut pb_distances: Vec<i64> = Vec::new();

    for (_line_index, line) in content.iter().enumerate() {
        // new line, flush
        let mut ss: VecDeque<String> = line
            .split(' ')
            .filter_map(|x| {
                if x.to_string().trim().len() > 0 {
                    Some(x.to_string())
                } else {
                    None
                }
            })
            .collect();

        let tt = ss.pop_front().unwrap();

        if tt == "Time:" {
            while let Some(item) = ss.pop_front() {
                pb_times.push(item.parse::<i64>().unwrap());
            }
        } else if tt == "Distance:" {
            while let Some(item) = ss.pop_front() {
                pb_distances.push(item.parse::<i64>().unwrap());
            }
        }
    }

    let race_time: i64 = pb_times
        .iter()
        .map(|x| x.to_string())
        .reduce(|a, b| a + &b)
        .unwrap()
        .parse()
        .unwrap();

    let race_max_distance: i64 = pb_distances
        .iter()
        .map(|x| x.to_string())
        .reduce(|a, b| a + &b)
        .unwrap()
        .parse()
        .unwrap();

    let mut number_ok = 0;
    for time_hold in 1..race_time {
        let remaining_time = race_time - time_hold;
        let local_distance = remaining_time * time_hold;
        if local_distance > race_max_distance {
            number_ok += 1;
        }
    }

    println!("res {} ", number_ok);

    // assert_eq!(pb_times.len(), pb_distances.len());
    // let race_count = pb_times.len();
    // let mut numbers: Vec<i64> = Vec::new();
    // for i in 0..race_count {
    //     let race_time = pb_times[i];
    //     let race_max_distance = pb_distances[i];
    //     let mut number_ok = 0;

    //     for time_hold in 1..race_time {
    //         let remaining_time = race_time - time_hold;
    //         let local_distance = remaining_time * time_hold;
    //         if local_distance > race_max_distance {
    //             number_ok += 1;
    //         }
    //     }
    //     numbers.push(number_ok);
    // }

    // let res = numbers.iter().copied().reduce(|a, b| a * b).unwrap();
    // println!("Final solution  {:?}", res);
}

fn main() {
    // Day 1 first problem
    let _line = String::new();
    // let _i: std::i32 = 50;
    // let file_path = "test_input.txt";
    let file_path = "input.txt";
    let content = read_file(file_path);
    process_content(content);
}
