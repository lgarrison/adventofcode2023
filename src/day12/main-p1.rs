#![allow(non_snake_case)]

use core::panic;
use std::{fs, vec};

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
    known_counts: (Vec<i64>,i64),
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
        write!(f, ", {:?}", self.known_counts)?;
        Ok(())
    }
}

fn get_known_counts(data: &Vec<State>) -> (Vec<i64>,i64) {
    let mut result = Vec::new();
    let end = data.iter().position(|&s| s == State::Unknown).unwrap_or(data.len());
    let mut grouplen = 0;
    for i in 0..end {
        match data[i] {
            State::Damaged => {
                grouplen += 1;
            },
            State::Operational => {
                if grouplen > 0 {
                    result.push(grouplen);
                    grouplen = 0;
                }
            },
            _ => panic!("Invalid state"),
        }
    }
    if grouplen > 0 && end == data.len() {
        result.push(grouplen);
        grouplen = 0;
    }
    (result, grouplen)
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

        let kc = get_known_counts(&data);
        Record { data,
                 target_counts,
                 known_counts: kc,
                }
    }

    fn get(&self, index: usize) -> State {
        self.data[index]
    }

    fn mark_copy(&self, index: usize, state: State) -> Record {
        let mut data = self.data.clone();
        data[index] = state;
        let kc = get_known_counts(&data);
        Record {
            data,
            target_counts: self.target_counts.clone(),
            known_counts: kc,
        }
    }

    fn transitions(&self, first: usize) -> (Vec<Record>, usize) {
        for i in first..self.data.len() {
            match self.get(i) {
                State::Unknown => {
                    return (vec![self.mark_copy(i, State::Operational),
                                self.mark_copy(i, State::Damaged)], i);
                },
                _ => (),
            }
        }
        return (Vec::new(),self.data.len());
    }

    fn all_known(&self) -> bool {
        self.data.iter().all(|&s| s != State::Unknown)
    }

    fn count_arrangements(&self) -> i64 {
        let mut result = 0;
        let mut queue = Vec::new();
        // let mut npush = 1;
        queue.push((self.clone(),0));
        while let Some((record,i)) = queue.pop() {
            let (transitions,j) = record.transitions(i);
            for t in transitions {
                // println!("t: {}", t);
                if t.known_counts.0 == t.target_counts && t.all_known() {
                    result += 1;
                } else if (t.known_counts.0.len() <= t.target_counts.len()) &&
                    (t.known_counts.0 == t.target_counts[..t.known_counts.0.len()]) {
                    
                    // if we have another group in progress, it must be smaller than the next target group
                    if (t.known_counts.1 > 0) 
                        && ((t.known_counts.0.len() == t.target_counts.len())
                            || (t.known_counts.1 > t.target_counts[t.known_counts.0.len()])) {
                        continue;
                    }

                    queue.push((t,j+1));  // still might be a match
                }
            }
        }
        // println!("npush: {}", npush);
        result
    }
}

fn part1(txt: &str) -> i64 {
    let records = txt
        .lines()
        // .skip(1)  // DEBUG
        // .take(1) // DEBUG
        .map(|line| Record::from_str(line, false))
        .collect::<Vec<Record>>();
    // println!("{:?}", records[..1].iter().map(|r| r.count_arrangements()).collect::<Vec<i64>>());
    records.iter().map(|r| {
        // print!(".");
        r.count_arrangements()
    }).sum()
}

fn part2(txt: &str) -> i64 {
    let records = txt
        .lines()
        // .skip(1)  // DEBUG
        // .take(1) // DEBUG
        .map(|line| Record::from_str(line, true))
        .collect::<Vec<Record>>();
    // println!("{:?}", records[..1].iter().map(|r| r.count_arrangements()).collect::<Vec<i64>>());
    records.iter().map(|r| {
        println!(".");
        r.count_arrangements()
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
