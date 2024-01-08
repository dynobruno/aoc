#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;

use std::io::BufRead;
use std::io::BufReader;

fn check_validity(input: &String, pattern: &Vec<usize>) -> (bool, Option<usize>) {
    let mut dashcount = 0;
    let mut patcount: Vec<usize> = Vec::new();

    for (idx, c) in input.chars().enumerate() {
        match c {
            '#' => {
                dashcount += 1;
            }
            '.' => {
                if dashcount > 0 {
                    // add to vector
                    patcount.push(dashcount);
                    dashcount = 0
                }
            }
            '?' => return (false, Some(idx)),
            _ => {
                assert!(false)
            }
        }
    }
    if dashcount > 0 {
        patcount.push(dashcount);
    }
    // handle exclamation checks here ?
    (patcount.eq(pattern), None)
}

fn compute_number(input: &String, pattern: &Vec<usize>) -> usize {
    // Return number of valid possibilities
    // .#.  1 => ok
    // .?.  1 => call recursive
    // ##.  1 => 0

    let (is_valid, has_exclamation_mark) = check_validity(input, pattern);

    if has_exclamation_mark.is_none() {
        if !is_valid {
            0
        } else {
            1
        }
    } else {
        let mut total = 0;
        let mark_pos = has_exclamation_mark.unwrap();
        let mut input2 = input.clone();
        // with #
        input2.replace_range(mark_pos..mark_pos + 1, "#");
        total += compute_number(&input2, pattern);
        // with .
        input2.replace_range(mark_pos..mark_pos + 1, ".");
        total += compute_number(&input2, pattern);
        total
    }
}

fn _smart_compute(
    memo: &mut HashMap<(usize, usize, usize), usize>,
    input: &[u8],
    pattern: &Vec<usize>,
    ipos: usize,
    ppos: usize,
    dash_count: usize,
) -> usize {
    // return number of possibilities

    let key = (ipos, ppos, dash_count);
    let res = memo.get(&key);
    if let Some(rr) = res {
        return *rr;
    }

    let debug = false;

    let mut res: Option<usize> = None;

    if ipos >= input.len() {
        // should not be here
        assert!(false);
    }

    if ppos >= pattern.len() {
        // expecting no more
        for i in ipos..input.len() {
            match input[i] {
                b'#' => {
                    res = Some(0);
                }
                b'.' | b'?' => {}
                _ => {
                    assert!(false)
                }
            }
        }
        if res.is_none() {
            res = Some(1)
        }
    } else if ipos == input.len() - 1 {
        // One byte left
        let cond1 = (ppos == (pattern.len() - 1)) & (pattern[ppos] == dash_count);
        let cond2 = (ppos == (pattern.len() - 1)) & (pattern[ppos] == dash_count + 1);
        res = Some(match input[ipos] {
            b'#' => {
                if cond2 {
                    1
                } else {
                    0
                }
            }
            b'.' => {
                if cond1 {
                    1
                } else {
                    0
                }
            }
            b'?' => {
                if cond1 | cond2 {
                    1
                } else {
                    0
                }
            }
            _ => {
                assert!(false);
                0
            }
        });
    } else {
        // Multiple bytes left
        let expected = pattern[ppos];
        res = Some(match input[ipos] {
            b'#' => {
                if dash_count < expected {
                    _smart_compute(memo, input, pattern, ipos + 1, ppos, dash_count + 1)
                } else {
                    0
                }
            }
            b'.' => {
                if dash_count > 0 {
                    if dash_count == expected {
                        _smart_compute(memo, input, pattern, ipos + 1, ppos + 1, 0)
                    } else {
                        0
                    }
                } else {
                    _smart_compute(memo, input, pattern, ipos + 1, ppos, dash_count)
                }
            }
            b'?' => {
                let dash_solu = {
                    if dash_count < expected {
                        _smart_compute(memo, input, pattern, ipos + 1, ppos, dash_count + 1)
                    } else {
                        0
                    }
                };
                let dot_solu = {
                    if dash_count > 0 {
                        if dash_count == expected {
                            let remains =
                                _smart_compute(memo, input, pattern, ipos + 1, ppos + 1, 0);
                            remains
                        } else {
                            0
                        }
                    } else {
                        _smart_compute(memo, input, pattern, ipos + 1, ppos, dash_count)
                    }
                };
                dash_solu + dot_solu
            }
            _ => {
                assert!(false);
                0
            }
        })
    }

    if let Some(x) = res {
        if debug {
            println!(
                " call _smart_compute {} {} {} => {}",
                ipos, ppos, dash_count, x
            );
        }
        memo.insert(key, x);
        return x;
    } else {
        assert!(false);
        0
    }
}

fn smart_compute(input: &String, pattern: &Vec<usize>) -> usize {
    println!("call smart_compute {} {:?}", input, pattern);
    let mut memo: HashMap<(usize, usize, usize), usize> = HashMap::new();
    _smart_compute(&mut memo, input.as_bytes(), pattern, 0, 0, 0)
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
    let mut total = 0;
    let part_two = true;
    for (_line_index, line) in content.iter().enumerate() {
        // new line, flush
        let mut ss = line.split(' ');
        let expr: String = ss.next().unwrap().to_string();
        let numbers: Vec<usize> = ss
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        if !part_two {
            let rr = compute_number(&expr, &numbers);
            println!("{} {:?} ==> {}", expr, numbers, rr);
            total += rr;
        } else {
            let mut input = expr.clone();
            let mut input_vec: Vec<usize> = numbers.clone();
            for _i in 0..4 {
                input += "?";
                input += &expr;
                input_vec.append(&mut numbers.clone());
            }
            // println!(
            //     "update from {} {:?} to {} {:?}",
            //     expr, numbers, input, input_vec
            // );
            let rr = smart_compute(&input, &input_vec);
            println!("{} {:?} ==> {}", expr, numbers, rr);
            total += rr;
        }
    }
    println!("{}", total);
}

fn main() {
    // Day 1 first problem
    let _line = String::new();
    // let _i: std::i32 = 50;
    // let file_path = "test_input.txt";
    let file_path = "input.txt";
    let content = read_file(file_path);
    println!("processing {}", file_path);
    process_content(content);
}

#[cfg(test)]
mod tests {
    use crate::{check_validity, compute_number, smart_compute};

    #[test]
    fn check_validity_works() {
        assert_eq!(check_validity(&"#".to_string(), &vec![1]), (true, None));
        assert_eq!(check_validity(&".".to_string(), &vec![1]), (false, None));
        assert_eq!(check_validity(&"?".to_string(), &vec![1]), (false, Some(0)));

        assert_eq!(
            check_validity(&"#.#.###".to_string(), &vec![1, 1, 3]),
            (true, None)
        );
        assert_eq!(
            check_validity(&"##.###".to_string(), &vec![2, 3]),
            (true, None)
        );
        assert_eq!(
            check_validity(&"##....###".to_string(), &vec![2, 3]),
            (true, None)
        );
        assert_eq!(
            check_validity(&"#??.###".to_string(), &vec![1, 1, 3]),
            (false, Some(1))
        );
    }

    #[test]
    fn compute_number_works() {
        assert_eq!(compute_number(&"#".to_string(), &vec![1]), 1);
        assert_eq!(compute_number(&".".to_string(), &vec![1]), 0);
        assert_eq!(compute_number(&".".to_string(), &vec![]), 1);
        assert_eq!(compute_number(&"?".to_string(), &vec![1]), 1);
        assert_eq!(compute_number(&"???".to_string(), &vec![1, 1]), 1);
        assert_eq!(compute_number(&"??.?".to_string(), &vec![1, 1]), 2);
    }

    #[test]
    fn smart_compute_works() {
        assert_eq!(smart_compute(&"#".to_string(), &vec![1]), 1);
        assert_eq!(smart_compute(&".".to_string(), &vec![1]), 0);
        assert_eq!(smart_compute(&".".to_string(), &vec![]), 1);
        assert_eq!(smart_compute(&"?".to_string(), &vec![1]), 1);
        assert_eq!(smart_compute(&"??".to_string(), &vec![1, 1]), 0);
        assert_eq!(smart_compute(&"??".to_string(), &vec![1]), 2);
        assert_eq!(smart_compute(&"???".to_string(), &vec![1, 1]), 1);

        assert_eq!(smart_compute(&"??.?".to_string(), &vec![1, 1]), 2);
        assert_eq!(smart_compute(&"??????#".to_string(), &vec![1]), 1);
    }
}
