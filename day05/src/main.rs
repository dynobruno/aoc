use std::cmp;
use std::fs::File;

use std::io::BufRead;
use std::io::BufReader;

struct RangeItem {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

struct RangeMap {
    from: String,
    to: String,
    items: Vec<RangeItem>,
}

#[derive(Clone, Debug)]
struct StuffRange {
    start: u64,
    len: u64,
    should_apply_offset: i64,
}

impl StuffRange {
    fn apply_offset_if_any(&mut self) {
        self.start = (self.start as i64 + self.should_apply_offset) as u64;
        self.should_apply_offset = 0;
    }

    fn cut(&self, cut_start: u64, cut_end: u64, offset: i64) -> Vec<StuffRange> {
        // assert_eq!(self.should_apply_offset, 0);
        let start = self.start;
        let end = self.start + self.len;

        let part1_start = cmp::min(start, cut_start);
        let part1_end = cmp::min(end, cut_start);
        let part1_len = part1_end as i64 - part1_start as i64;

        let part2_start = cmp::max(cut_start, start);
        let part2_end = cmp::min(cut_end, end);
        let part2_len = part2_end as i64 - part2_start as i64;

        let part3_start = cmp::max(cut_end, start);
        let part3_end = cmp::max(cut_end, end);
        let part3_len = part3_end as i64 - part3_start as i64;

        let mut res: Vec<StuffRange> = Vec::new();

        if part1_len > 0 {
            res.push(StuffRange {
                start: part1_start,
                len: part1_len as u64,
                should_apply_offset: self.should_apply_offset,
            });
        }
        if part2_len > 0 {
            res.push(StuffRange {
                start: part2_start,
                len: part2_len as u64,
                should_apply_offset: offset + self.should_apply_offset,
            });
        }
        if part3_len > 0 {
            res.push(StuffRange {
                start: part3_start,
                len: part3_len as u64,
                should_apply_offset: self.should_apply_offset,
            });
        }

        let total_len: u64 = res.iter().map(|x| x.len).sum();
        assert_eq!(total_len, self.len);

        res
    }
}

impl RangeMap {
    fn new(first_line: &String) -> RangeMap {
        let part = first_line.trim().replace(" map:", "");
        let mut ss = part.split("-to-");
        let from_str = ss.next().unwrap().to_string();
        let to_str = ss.next().unwrap().to_string();

        println!("New rangemap {} {}", from_str, to_str);

        RangeMap {
            from: from_str,
            to: to_str,
            items: Vec::new(),
        }
    }

    fn _convert(&self, id: u64) -> u64 {
        for item in &self.items {
            let mut is_in_range = item.source_start <= id;
            is_in_range &= (item.source_start + item.length) > id;
            if is_in_range {
                let offset: i64 = item.destination_start as i64 - item.source_start as i64;
                return (id as i64 + offset) as u64;
            }
        }
        id
    }

    fn convert_stuff_range(&self, id: &StuffRange) -> Vec<StuffRange> {
        let mut res = vec![id.clone()];

        for item in &self.items {
            let offset: i64 = item.destination_start as i64 - item.source_start as i64;
            let item_start = item.source_start;
            let item_end = item.source_start + item.length;

            let mut newres: Vec<StuffRange> = Vec::new();
            for stuff in res.iter() {
                let mut new_stuff = stuff.cut(item_start, item_end, offset);
                newres.append(&mut new_stuff);
            }
            res.clear();
            res.append(&mut newres);
        }

        for rr in res.iter_mut() {
            rr.apply_offset_if_any()
        }

        res
    }

    fn parse(&mut self, next_line: &String) -> bool {
        if next_line.trim().is_empty() {
            false
        } else {
            let mut ss = next_line.split(" ");
            let dest_start: u64 = ss.next().unwrap().to_string().parse().unwrap();
            let source_start: u64 = ss.next().unwrap().to_string().parse().unwrap();
            let range_len: u64 = ss.next().unwrap().to_string().parse().unwrap();
            self.items.push(RangeItem {
                destination_start: dest_start,
                source_start: source_start,
                length: range_len,
            });
            true
        }
    }
}

fn _try_convert(ranges: &Vec<RangeMap>, from: &String, id: u64) -> Option<(String, u64)> {
    for range in ranges {
        if range.from == *from {
            return Some((range.to.clone(), range._convert(id)));
        }
    }
    None
}

fn try_convert_range(
    ranges: &Vec<RangeMap>,
    from: &String,
    id: &Vec<StuffRange>,
) -> Option<(String, Vec<StuffRange>)> {
    for range in ranges {
        if range.from == *from {
            let mut res: Vec<StuffRange> = Vec::new();
            for ii in id.iter() {
                let mut v = range.convert_stuff_range(ii);
                res.append(&mut v);
            }
            return Some((range.to.clone(), res));
        }
    }
    None
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
    let mut seeds: Vec<StuffRange> = Vec::new();
    let mut ranges: Vec<RangeMap> = Vec::new();
    let mut current_rangemap: Option<RangeMap> = None;

    for (line_index, line) in content.iter().enumerate() {
        // new line, flush
        if line_index == 0 {
            // first line
            let aa = line.trim().replace("seeds: ", "");
            // let seeds = aa.split(" ").map(|x| x.parse::<u64>().unwrap()).collect();
            let mut ss = aa.split(" ");
            while let Some(next_elem) = ss.next() {
                let range_start = next_elem.to_string().parse::<u64>().unwrap();
                let range_len = ss.next().unwrap().to_string().parse::<u64>().unwrap();
                seeds.push(StuffRange {
                    start: range_start,
                    len: range_len,
                    should_apply_offset: 0,
                });
            }
        } else {
            if line.trim().len() == 0 {
                if current_rangemap.is_some() {
                    ranges.push(current_rangemap.take().unwrap());
                }
                continue;
            } else {
                match &mut current_rangemap {
                    Some(x) => {
                        if x.parse(line) {
                            continue;
                        } else {
                            ranges.push(current_rangemap.take().unwrap());
                        }
                    }
                    None => current_rangemap = Some(RangeMap::new(line)),
                }
            }
        }
    }
    if current_rangemap.is_some() {
        ranges.push(current_rangemap.take().unwrap());
    }

    //
    let mut locations: Vec<StuffRange> = Vec::new();

    for (_idx, seed) in seeds.iter().enumerate() {
        let mut cunit = "seed".to_string();
        let mut cid: Vec<StuffRange> = vec![seed.clone()];
        while cunit != "location" {
            match try_convert_range(&ranges, &cunit, &cid) {
                Some((nunit, nid)) => {
                    println!("Found from {} {:?} to  {} {:?}", cunit, cid, nunit, nid);
                    cunit = nunit;
                    cid = nid;
                }
                None => {
                    // println!("Cannot find {} {}", cunit, cid);
                    assert!(false);
                }
            }
        }
        locations.append(&mut cid);
    }

    let minn = locations.iter().map(|x| x.start).min().unwrap();

    println!("Final  {}", minn);
}

fn main() {
    // Day 1 first problem
    let _line = String::new();
    // let _i: std::i64 = 50;
    // let file_path = "test_input.txt";
    let file_path = "input.txt";
    let content = read_file(file_path);
    process_content(content);
}
