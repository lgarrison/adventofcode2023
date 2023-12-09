#![allow(non_snake_case)]

use std::fs;

fn extrapolate(vals: &Vec<i64>) -> i64 {
    if vals.iter().all(|&x| x == vals[0]) {
        vals[0]
    } else {
        extrapolate(
            &vals.iter().skip(1).zip(vals.iter())
                .map(|(x,y)| x-y).collect(),
        ) + vals.last().unwrap()
    }
}

fn part1(txt: &str) -> i64 {
    txt.lines()
        .map(|l| l.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect())
        .map(|v: Vec<i64>| extrapolate(&v))
        .sum()
}

fn part2(txt: &str) -> i64 {
    txt.lines()
        .map(|l| l.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect())
        .map(|v: Vec<i64>| extrapolate(&v.iter().rev().cloned().collect::<Vec<_>>()))
        .sum()
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
