#![allow(dead_code)]

use std::fs::File;

use std::io::BufRead;
use std::io::BufReader;

use std::ops::Add;
use std::ops::Sub;

struct Matrix<T> {
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Clone,
{
    fn new(line_count: i64, row_count: i64, init_value: T) -> Matrix<T> {
        let mut v: Vec<Vec<T>> = Vec::new();
        v.resize_with(line_count as usize, || {
            let mut a = Vec::new();
            a.resize_with(row_count as usize, || init_value.clone());
            a
        });
        Matrix { data: v }
    }

    fn get(&self, point: &(i64, i64)) -> &T {
        let item = &self.data[point.0 as usize][point.1 as usize];
        item
    }

    fn get_mut(&mut self, point: &(i64, i64)) -> &mut T {
        let item = &mut self.data[point.0 as usize][point.1 as usize];
        item
    }

    fn set(&mut self, point: &(i64, i64), value: T) {
        self.data[point.0 as usize][point.1 as usize] = value;
    }
}

trait Coordinate<T> {
    fn is_same_line(self, other: &(T, T)) -> bool;
    fn is_same_row(self, other: &(T, T)) -> bool;
    fn line(self) -> T;
    fn row(self) -> T;
    fn up(self) -> (T, T);
    fn down(self) -> (T, T);
    fn left(self) -> (T, T);
    fn right(self) -> (T, T);
}

impl<T> Coordinate<T> for (T, T)
where
    T: Eq + Ord + Add<i64, Output = T> + Sub<i64, Output = T>,
{
    fn is_same_line(self, other: &(T, T)) -> bool {
        self.0 == other.0
    }
    fn is_same_row(self, other: &(T, T)) -> bool {
        self.1 == other.1
    }
    fn line(self) -> T {
        self.0
    }
    fn row(self) -> T {
        self.1
    }
    fn up(self) -> (T, T) {
        (self.0 - 1, self.1)
    }
    fn down(self) -> (T, T) {
        (self.0 + 1, self.1)
    }
    fn left(self) -> (T, T) {
        (self.0, self.1 - 1)
    }
    fn right(self) -> (T, T) {
        (self.0, self.1 + 1)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Pipe {
    symbol: char,
    rank: i64,
}

impl Pipe {
    fn new(symbol: char) -> Pipe {
        Pipe {
            symbol: symbol,
            rank: 0,
        }
    }

    fn next_position(
        &self,
        prev_position: &(i64, i64),
        current_position: &(i64, i64),
    ) -> Option<(i64, i64)> {
        if self.symbol == '.' {
            None
        } else if self.symbol == '|' {
            if prev_position.is_same_row(current_position) {
                // same row
                let diff = current_position.0 - prev_position.0;
                if diff.abs() == 1 {
                    Some((current_position.0 + diff, current_position.1))
                } else {
                    None
                }
            } else {
                // different row, not ok
                None
            }
        } else if self.symbol == '-' {
            if prev_position.is_same_line(current_position) {
                // same line
                let diff = current_position.1 - prev_position.1;
                if diff.abs() == 1 {
                    Some((current_position.0, current_position.1 + diff))
                } else {
                    None
                }
            } else {
                // different row, not ok
                None
            }
        } else if self.symbol == 'L' {
            if current_position.right() == *prev_position {
                Some(current_position.up())
            } else if current_position.up() == *prev_position {
                Some(current_position.right())
            } else {
                None
            }
        } else if self.symbol == 'J' {
            if current_position.left() == *prev_position {
                Some(current_position.up())
            } else if current_position.up() == *prev_position {
                Some(current_position.left())
            } else {
                None
            }
        } else if self.symbol == '7' {
            if current_position.left() == *prev_position {
                Some(current_position.down())
            } else if current_position.down() == *prev_position {
                Some(current_position.left())
            } else {
                None
            }
        } else if self.symbol == 'F' {
            if current_position.right() == *prev_position {
                Some(current_position.down())
            } else if current_position.down() == *prev_position {
                Some(current_position.right())
            } else {
                None
            }
        } else if self.symbol == '.' {
            None
        } else if self.symbol == 'S' {
            Some(*current_position)
        } else {
            None
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

fn get_first_possible(table: &Matrix<Pipe>, position: &(i64, i64)) -> Option<(i64, i64)> {
    let mut possible_first = vec![position.down(), position.right(), position.left()];
    if position.0 > 0 {
        possible_first.push(position.up());
    }

    for item in possible_first.iter() {
        let case = table.get(item);
        match case.next_position(&position, item) {
            Some(_) => return Some(*item),
            None => {}
        }
    }
    None
}

fn process_content(content: Vec<String>) {
    let line_count = content.len() as i64;
    let row_count = content[0].len() as i64;
    let mut table: Matrix<Pipe> = Matrix::new(line_count, row_count, Pipe::new('.'));
    let mut spos: Option<(i64, i64)> = None;

    for (line_index, line) in content.iter().enumerate() {
        // new line, flush
        for (row_index, c) in line.chars().enumerate() {
            if c == 'S' {
                spos = Some((line_index as i64, row_index as i64));
            }
            let new_pipe = Pipe::new(c);
            table.set(&(line_index as i64, row_index as i64), new_pipe);
        }
    }

    println!("Initial positions {:?}", spos);

    let mut prev_position = spos.unwrap();
    let mut current_position = get_first_possible(&table, &prev_position).unwrap();
    let mut count = 1;

    table.get_mut(&prev_position).rank = count;
    count += 1;

    loop {
        let case = table.get_mut(&current_position);
        case.rank = count;
        match case.next_position(&prev_position, &current_position) {
            Some(next) => {
                if next == current_position {
                    // arrived
                    break;
                } else {
                    count += 1;
                    prev_position = current_position;
                    current_position = next;
                }
            }
            None => {
                assert!(false);
            }
        }
    }

    println!("Final count {}", (count + 1) / 2);
    let mut area_count = 0;
    for i in 0..line_count {
        let mut is_inside_loop = false;
        let mut wait_south = false;
        let mut wait_north = false;
        println!("");
        for j in 0..row_count {
            let case = table.get(&(i, j));
            let is_loop = case.rank > 0;

            if is_loop {
                let mut connect_north = false;
                let mut connect_south = false;
                match case.symbol {
                    '|' => {
                        connect_north = true;
                        connect_south = true;
                    }
                    'L' => connect_north = true,
                    'J' => connect_north = true,
                    '7' => connect_south = true,
                    'F' => connect_south = true,
                    'S' => {
                        connect_north = true;
                        connect_south = true;
                    }
                    _ => {}
                }
                if connect_north & connect_south {
                    assert_eq!(wait_south, false);
                    assert_eq!(wait_north, false);
                    is_inside_loop = !is_inside_loop;
                } else {
                    if connect_north {
                        if wait_north {
                            assert_eq!(wait_south, false);
                            is_inside_loop = !is_inside_loop;
                            wait_north = false;
                        } else {
                            wait_south = !wait_south;
                        }
                    }
                    if connect_south {
                        if wait_south {
                            assert_eq!(wait_north, false);
                            is_inside_loop = !is_inside_loop;
                            wait_south = false;
                        } else {
                            wait_north = !wait_north;
                        }
                    }
                }
                print!("{}", case.symbol)
            } else {
                if is_inside_loop {
                    area_count += 1;
                    print!("{}", "*")
                } else {
                    print!("{}", ".")
                }
            }
        }
        // println!("line {} area {}", i, area_count);
    }
    println!("Final area {}", area_count);
}

fn main() {
    // Day 1 first problem
    let _line = String::new();
    // let _i: std::i32 = 50;
    // let file_path = "test_input3.txt";
    let file_path = "input.txt";
    let content = read_file(file_path);
    process_content(content);
}
