#![allow(dead_code)]
use std::collections::HashMap;
use std::fs::File;

use std::cmp::Ordering;
use std::io::BufRead;
use std::io::BufReader;
#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Clone, Debug, PartialEq)]
struct CardHand {
    hand: String,
    bid: u32,
}

impl CardHand {
    fn new(value: &str) -> CardHand {
        assert_eq!(value.len(), 5);
        CardHand {
            hand: value.to_string(),
            bid: 0,
        }
    }

    fn new_with_bid(value: &str, bid: u32) -> CardHand {
        assert_eq!(value.len(), 5);
        CardHand {
            hand: value.to_string(),
            bid: bid,
        }
    }

    fn compare(&self, other: &CardHand) -> Ordering {
        let my_kind = self.get_kind_with_joker();
        let other_kind = other.get_kind_with_joker();

        if my_kind > other_kind {
            Ordering::Greater
        } else if my_kind < other_kind {
            Ordering::Less
        } else {
            self.compare_high(other)
        }
    }

    fn compare_high(&self, other: &CardHand) -> Ordering {
        let mut values: HashMap<char, u32> = HashMap::new();

        values.insert('J', 1); // Part two change

        for i in 2..10 {
            values.insert(char::from_digit(i, 10).unwrap(), i);
        }
        values.insert('T', 10);
        values.insert('Q', 12);
        values.insert('K', 13);
        values.insert('A', 14);

        for (a, b) in Iterator::zip(self.hand.chars(), other.hand.chars()) {
            let av = values.get(&a).unwrap();
            let bv = values.get(&b).unwrap();
            let res = av.cmp(bv);
            if res != Ordering::Equal {
                return res;
            }
        }
        Ordering::Equal
    }

    fn get_kind_with_joker(&self) -> HandKind {
        let mut count: HashMap<char, usize> = HashMap::new();
        let mut joker_count: usize = 0;
        for i in self.hand.chars() {
            let newval = count.get(&i).unwrap_or(&0) + 1;
            if i == 'J' {
                joker_count += 1;
            } else {
                count.insert(i, newval);
            }
        }

        let mut countcount: Vec<usize> = count
            .iter()
            .map(|(a, b)| if *a == 'J' { *b } else { *b })
            .collect();
        countcount.sort_by(|a, b| b.cmp(a));

        if countcount.len() == 0 {
            countcount.push(0);
        }
        countcount[0] += joker_count;

        if countcount[0] == 5 {
            HandKind::FiveKind
        } else if countcount[0] == 4 {
            HandKind::FourKind
        } else if countcount[0] == 3 {
            if countcount[1] == 2 {
                HandKind::FullHouse
            } else {
                HandKind::ThreeKind
            }
        } else if countcount[0] == 2 {
            if countcount[1] == 2 {
                HandKind::TwoPair
            } else {
                HandKind::OnePair
            }
        } else {
            HandKind::HighCard
        }
    }

    fn get_kind(&self) -> HandKind {
        let mut count: HashMap<char, usize> = HashMap::new();
        for i in self.hand.chars() {
            let newval = count.get(&i).unwrap_or(&0) + 1;
            count.insert(i, newval);
        }

        let mut countcount: Vec<usize> = count.values().copied().collect();
        countcount.sort_by(|a, b| b.cmp(a));

        if countcount[0] == 5 {
            HandKind::FiveKind
        } else if countcount[0] == 4 {
            HandKind::FourKind
        } else if countcount[0] == 3 {
            if countcount[1] == 2 {
                HandKind::FullHouse
            } else {
                HandKind::ThreeKind
            }
        } else if countcount[0] == 2 {
            if countcount[1] == 2 {
                HandKind::TwoPair
            } else {
                HandKind::OnePair
            }
        } else {
            HandKind::HighCard
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
    let mut deck: Vec<CardHand> = Vec::new();
    for (_line_index, line) in content.iter().enumerate() {
        let mut ss = line.trim().split(' ');
        let card_str = ss.next().unwrap();
        let card_bid: u32 = ss.next().unwrap().parse().unwrap();
        let card = CardHand::new_with_bid(card_str, card_bid);
        deck.push(card);
    }

    deck.sort_by(|a, b| a.compare(b));

    let mut res: i64 = 0;
    for (idx, card) in deck.iter().enumerate() {
        let rank = idx + 1;
        res += rank as i64 * card.bid as i64;
    }
    println!("Final {}", res);
}

fn main() {
    // Day 1 first problem
    let _line = String::new();
    // let _i: std::i32 = 50;
    //let file_path = "test_input.txt";
    let file_path = "input.txt";
    let content = read_file(file_path);
    process_content(content);
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::{CardHand, HandKind};

    #[test]
    fn test_kind() {
        assert_eq!(CardHand::new("32T3K").get_kind(), HandKind::OnePair);
        assert_eq!(CardHand::new("T55J5").get_kind(), HandKind::ThreeKind);
        assert_eq!(CardHand::new("KK777").get_kind(), HandKind::FullHouse);
        assert_eq!(CardHand::new("QAQJA").get_kind(), HandKind::TwoPair);
        assert_eq!(CardHand::new("AKKKK").get_kind(), HandKind::FourKind);
        assert_eq!(CardHand::new("23456").get_kind(), HandKind::HighCard);
    }

    #[test]
    fn test_kind_with_jocker() {
        assert_eq!(
            CardHand::new("32T3K").get_kind_with_joker(),
            HandKind::OnePair
        );
        assert_eq!(
            CardHand::new("T55J5").get_kind_with_joker(),
            HandKind::FourKind
        );
        assert_eq!(
            CardHand::new("KK677").get_kind_with_joker(),
            HandKind::TwoPair
        );
        assert_eq!(
            CardHand::new("KTJJT").get_kind_with_joker(),
            HandKind::FourKind
        );
        assert_eq!(
            CardHand::new("QQQJA").get_kind_with_joker(),
            HandKind::FourKind
        );
        assert_eq!(
            CardHand::new("23456").get_kind_with_joker(),
            HandKind::HighCard
        );
        assert_eq!(
            CardHand::new("JJJJJ").get_kind_with_joker(),
            HandKind::FiveKind
        );
    }

    #[test]
    fn test_high() {
        assert_eq!(
            CardHand::new("32T3K").compare_high(&CardHand::new("32T3K")),
            Ordering::Equal
        );

        assert_eq!(
            CardHand::new("32T3K").compare_high(&CardHand::new("33T3K")),
            Ordering::Less
        );

        assert_eq!(
            CardHand::new("32K3K").compare_high(&CardHand::new("32T3K")),
            Ordering::Greater
        );
    }

    #[test]
    fn test_compare() {
        assert_eq!(
            CardHand::new("TTT22").compare(&CardHand::new("2TTTT")),
            Ordering::Less
        );
    }
}
