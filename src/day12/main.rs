#![allow(non_snake_case)]

use core::panic;
use std::{fs, vec, env, collections::HashMap, f32::consts::E};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
struct Record {
    data: Vec<State>,
    target_counts: Vec<i64>,
}

impl State {
    fn from_char(c: char) -> State {
        match c {
            '.' => State::Operational,
            '#' => State::Damaged,
            '?' => State::Unknown,
            _ => panic!("Invalid state"),
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            State::Operational => write!(f, "."),
            State::Damaged => write!(f, "#"),
            State::Unknown => write!(f, "?"),
        }
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for s in &self.data {
            write!(f, "{}", s)?;
        }
        write!(f, ", {:?}", self.target_counts)?;
        Ok(())
    }
}

impl Record {
    fn from_str(line: &str, p2: bool) -> Record {
        let mut parts = line.split_whitespace();
        let mut s = parts.next().unwrap().to_owned();
        if p2 {
            s = vec![&s[..]].repeat(5).join("?");
        }
        let data = s
                .chars()
                .map(|c| State::from_char(c))
            .collect::<Vec<State>>();
        let target_counts = std::iter::repeat(
            parts
                .next()
                .unwrap()
                .split(",")
                .map(|s| s.parse::<i64>().unwrap())
            ).take(if p2 { 5 } else { 1 })
            .flatten()
            .collect::<Vec<i64>>();

        Record { data,
                 target_counts,
                }
    }

    fn get(&self, index: usize) -> State {
        self.data[index]
    }

    fn count_arrangements(&self, run_size: i64, memos: &mut HashMap<(Record,i64),i64>) -> i64 {
        if let Some(&result) = memos.get(&(self.clone(), run_size)) {
            // println!("hit");
            return result;
        }

        if self.data.len() == 0 {
            if (self.target_counts.len() == 1 && self.target_counts[0] == run_size)
                || (self.target_counts.len() == 0 && run_size == 0) {
                memos.insert((self.clone(), run_size), 1);
                return 1;
            } else {
                memos.insert((self.clone(), run_size), 0);
                return 0;
            }
        }

        if self.target_counts.len() == 0 {
            if self.data.iter().all(|s| s == &State::Operational || s == &State::Unknown) {
                memos.insert((self.clone(), run_size), 1);
                return 1;
            } else {
                memos.insert((self.clone(), run_size), 0);
                return 0;
            }
        }

        match self.get(0) {
            State::Operational => {
                let off = if run_size == self.target_counts[0] {
                    1
                } else if run_size == 0 {
                    0
                } else {
                    memos.insert((self.clone(), run_size), 0);
                    return 0;
                };
                let next = Record {
                    data: self.data[1..].to_vec(),
                    target_counts: self.target_counts[off..].to_vec(),
                };
                let res = next.count_arrangements(0, memos);
                memos.insert((self.clone(), run_size), res);
                return res;
            },
            State::Damaged => {
                let next = Record {
                    data: self.data[1..].to_vec(),
                    target_counts: self.target_counts.clone(),
                };
                let res = next.count_arrangements(run_size + 1, memos);
                memos.insert((self.clone(), run_size), res);
                return res;
            },
            State::Unknown => {
                let next1 = Record {
                    data: { let mut d = self.data.clone(); d[0] = State::Operational; d },
                    target_counts: self.target_counts.clone(),
                };
                let next2 = Record {
                    data: { let mut d = self.data.clone(); d[0] = State::Damaged; d },
                    target_counts: self.target_counts.clone(),
                };
                let res = next1.count_arrangements(run_size, memos) +
                    next2.count_arrangements(run_size, memos);
                memos.insert((self.clone(), run_size), res);
                return res;
            },
        }
    }
}

fn part1(txt: &str) -> i64 {
    let mut memos = HashMap::new();
    let records = txt
        .lines()
        // .skip(1)  // DEBUG
        // .take(1) // DEBUG
        .map(|line| Record::from_str(line, false))
        .collect::<Vec<Record>>();
    // println!("{:?}", records[..1].iter().map(|r| r.count_arrangements()).collect::<Vec<i64>>());
    records.iter().map(|r| {
        // print!(".");
        r.count_arrangements(0, &mut memos)
    }).sum()
}

fn part2(txt: &str) -> i64 {
    let mut memos = HashMap::new();
    let records = txt
        .lines()
        .map(|line| Record::from_str(line, true))
        .collect::<Vec<Record>>();
    // println!("{:?}", records[..1].iter().map(|r| r.count_arrangements()).collect::<Vec<i64>>());
    records.iter().map(|r| {
        // println!(".");
        r.count_arrangements(0, &mut memos)
    }).sum()
}

fn main() {
    let dayX = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + dayX + "/input.txt";
    // let path = String::from(root) + "/src/" + dayX + "/test1.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", dayX);
    println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}
