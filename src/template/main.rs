#![allow(non_snake_case)]

use std::fs;

fn part1(txt: &str) -> i64 {
    0
}

fn part2(txt: &str) -> i64 {
    0
}

fn main() {
    let dayX = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    //let path = String::from(root) + "/src/" + dayX + "/input.txt";
    let path = String::from(root) + "/src/" + dayX + "/test1.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", dayX);
    println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}
