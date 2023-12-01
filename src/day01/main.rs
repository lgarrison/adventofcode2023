#![allow(non_snake_case)]

use std::fs;

fn part1(txt: &str) -> u32 {
    let i = txt.lines().map(
        |line| line.chars().filter_map(
            |c| c.to_digit(10)
        )
    );
    i.map(|j| j.clone().next().unwrap()*10
        + j.last().unwrap()
    )
    .sum()
}

fn replace_at(word: &str, i: usize) -> Option<String> {
    let names = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut res = String::from(word);
    for j in 0..names.len() {
        if word[i..].starts_with(names[j]) {
            res.replace_range(i..(i+names[j].len()), &j.to_string());
            return Some(res);
        }
    }
    return None;
}

fn replace_one_leading(word: &str, rev: bool) -> String {
    for i in 0..word.len() {
        if let Some(res) = replace_at(&word, if rev { word.len()-i-1 } else { i }) {
            return res;
        }
    }
    return word.to_string();
}

fn part2(txt: &str) -> u32 {
    let i = txt.lines().map(
        |line| replace_one_leading(
            &replace_one_leading(line, false),
            true)
            .chars().filter_map(
            |c| c.to_digit(10)
        ).collect::<Vec<u32>>()
    );
    i.map(|j| j.first().unwrap()*10
        + j.last().unwrap()
    )
    .sum()
}

fn main() {
    let dayX = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + dayX + "/input.txt";
    // let path = String::from(root) + "/src/" + dayX + "/test1.txt";
    // let path = String::from(root) + "/src/" + dayX + "/test2.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", dayX);
    println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}
