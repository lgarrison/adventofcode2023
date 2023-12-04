#![allow(non_snake_case)]

use std::fs;

fn part1(txt: &str) -> i64 {
    txt.lines().map(|l| {
        let mut c = l.split([':', '|']).skip(1).map(|s| s.split_whitespace());
        let left = c.next().unwrap();
        let right = c.next().unwrap();
        let count = left.filter(|n| right.clone().find(|s| s == n).is_some()).count() as u32;
        if count >= 1 { 2i64.pow(count - 1) } else { 0 }
    }).sum()
}

fn part2(txt: &str) -> usize {
    let mut nmatch = txt.lines().map(|l| {
        let mut c = l.split([':', '|']).skip(1).map(|s| s.split_whitespace());
        let left = c.next().unwrap();
        let right = c.next().unwrap();
        left.filter(|n| right.clone().find(|s| s == n).is_some()).count()
    }).collect::<Vec<usize>>();

    for i in (0..nmatch.len()).rev() {
        nmatch[i] += nmatch[(i+1)..(i + nmatch[i] + 1)].iter().sum::<usize>();
    }

    nmatch.iter().sum::<usize>() + nmatch.len()
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
