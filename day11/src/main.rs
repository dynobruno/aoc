#![allow(dead_code)]
use std::cmp;
use std::fs::File;

use std::io::BufRead;
use std::io::BufReader;

struct Matrix<T> {
    data: Vec<Vec<T>>,
    line_count: i64,
    row_count: i64,
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
        Matrix {
            data: v,
            row_count: row_count,
            line_count: line_count,
        }
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

    fn insert_row(&mut self, position: i64, init_value: T) {
        for i in 0..self.data.len() {
            self.data[i].insert(position as usize, init_value.clone())
        }
        self.row_count += 1;
    }

    fn insert_line(&mut self, position: i64, init_value: T) {
        let mut v: Vec<T> = Vec::new();
        v.resize_with(self.data[0].len(), || init_value.clone());
        self.data.insert(position as usize, v);
        self.line_count += 1;
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
    let mut table: Matrix<i32> = Matrix::new(content.len() as i64, content[0].len() as i64, 0);
    let mut galaxy_count = 0;

    for (line_index, line) in content.iter().enumerate() {
        // new line, flush
        for (row_index, c) in line.chars().enumerate() {
            if c == '#' {
                galaxy_count += 1;
                table.set(&(line_index as i64, row_index as i64), galaxy_count);
            }
        }
    }

    // expension
    let mut empty_lines = Vec::new();
    let mut empty_rows = Vec::new();
    {
        let line_count = table.line_count;
        for iline in 0..line_count {
            let ridx = line_count - iline - 1;
            let all_zero = table.data[ridx as usize].iter().all(|x| *x == 0);
            if all_zero {
                println!("New empty line {}", ridx);
                empty_lines.push(ridx);
            }
        }

        let row_count = table.row_count;
        for irow in 0..row_count {
            let ridx = row_count - irow - 1;
            let mut all_zero = true;
            for iline in 0..table.line_count {
                if *table.get(&(iline, ridx)) > 0 {
                    all_zero = false;
                }
            }
            if all_zero {
                println!("New empty row {}", ridx);
                empty_rows.push(ridx);
            }
        }
    }

    // Scan galaxies
    let mut galaxies: Vec<(i64, i64)> = Vec::new();
    for i in 0..table.line_count {
        for j in 0..table.row_count {
            let v = table.get(&(i, j));
            if *v > 0 {
                galaxies.push((i as i64, j as i64))
            }
        }
    }
    // display
    let should_display = false;
    println!("");
    if should_display {
        for i in 0..table.line_count {
            for j in 0..table.row_count {
                let v = table.get(&(i, j));
                if *v == 0 {
                    print!("{}", ".");
                } else {
                    print!("{}", v);
                }
            }
            println!("");
        }
    }

    // pair of galaxy
    let mut sum = 0;

    for idx in 0..galaxies.len() {
        for idx2 in (idx + 1)..galaxies.len() {
            let (g1x, g1y) = galaxies[idx];
            let (g2x, g2y) = galaxies[idx2];

            let x_mult_count: i64 = (cmp::min(g1x + 1, g2x + 1)..cmp::max(g1x, g2x))
                .map(|x| if empty_lines.contains(&x) { 1 } else { 0 })
                .sum();
            let y_mult_count: i64 = (cmp::min(g1y + 1, g2y + 1)..cmp::max(g1y, g2y))
                .map(|y| if empty_rows.contains(&y) { 1 } else { 0 })
                .sum();

            let mut distance = (g1x - g2x).abs() + (g1y - g2y).abs();
            let mult = 1000000;
            if x_mult_count > 0 {
                distance += x_mult_count * (mult - 1);
            }
            if y_mult_count > 0 {
                distance += y_mult_count * (mult - 1);
            }
            sum += distance;
            // println!(
            //     "Distance between {} and {} is {}  {:?} {:?}",
            //     idx + 1,
            //     idx2 + 1,
            //     distance,
            //     galaxies[idx],
            //     galaxies[idx2],
            // );
        }
    }

    println!("Galaxy count {}", galaxy_count);
    println!("Galaxy distance sum {}", sum); // 9599070 // 842646756432
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
