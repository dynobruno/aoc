#![allow(dead_code)]
use std::collections::HashMap;
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

struct PathItem {
    left: String,
    right: String,
}

fn process_content(content: Vec<String>) {
    let mut commands: String = String::new();
    let mut way: HashMap<String, PathItem> = HashMap::new();
    for (line_index, line) in content.iter().enumerate() {
        // new line, flush
        if line_index == 0 {
            commands = line.trim().to_string();
        } else {
            let ll = line.trim().to_string();
            if ll.len() == 0 {
                continue;
            }
            let mut ss = ll.split(" = ");
            let key = ss.next().unwrap();
            let second = ss.next().unwrap();
            let mut ss = second.split(", ");
            let f1 = ss.next().unwrap().to_string().replace("(", "");
            let f2: String = ss.next().unwrap().to_string().replace(")", "");
            // println!("{} {} {}", key, f1, f2);
            way.insert(
                key.to_string(),
                PathItem {
                    left: f1,
                    right: f2,
                },
            );
        }
    }

    // PART 1
    // let mut position = "AAA".to_string();
    // let mut count = 0;
    // while position != "ZZZ" {
    //     for c in commands.chars() {
    //         let instructions = way.get(&position).unwrap();
    //         count += 1;
    //         if c == 'L' {
    //             position = instructions.left.clone();
    //         } else if c == 'R' {
    //             position = instructions.right.clone();
    //         } else {
    //             assert!(false);
    //         }
    //         if position == "ZZZ" {
    //             break;
    //         }
    //     }
    // }
    // println!("count {}", count);

    let mut positions: Vec<String> = way
        .keys()
        .filter_map(|a| if a.ends_with("A") { Some(a) } else { None })
        .cloned()
        .collect();
    let mut valid = positions.iter().all(|x| x.ends_with('Z'));
    let mut count = 0;
    println!("Initial position {:?}", positions);
    println!("Initial commands {:?}", commands);

    let mut fist: Vec<u32> = Vec::new();
    fist.resize(positions.len(), 0);

    while !valid {
        for c in commands.chars() {
            count += 1;
            valid = true;
            for i in 0..positions.len() {
                let position = &positions[i];
                let instructions = way.get(position).unwrap();
                if c == 'L' {
                    positions[i] = instructions.left.clone();
                } else if c == 'R' {
                    positions[i] = instructions.right.clone();
                } else {
                    assert!(false);
                }
                if !positions[i].ends_with('Z') {
                    valid = false;
                } else {
                    if fist[i] == 0 {
                        println!("pos {} multiple {}", i, count);
                        fist[i] = count;
                        if fist.iter().all(|x| *x > 0) {
                            valid = true;
                            break;
                        }
                    } else {
                        continue;
                    }
                }
            }
            if valid {
                break;
            }
        }
    }

    println!("final{:?}", count);

    ()
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
