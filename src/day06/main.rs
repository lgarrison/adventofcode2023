#![allow(non_snake_case)]

use std::fs;

fn part1(txt: &str) -> i64 {
    let mut lines = txt.lines();
    let times = lines.next().unwrap().split_whitespace().skip(1).map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let dists = lines.next().unwrap().split_whitespace().skip(1).map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    times.iter().zip(dists.iter()).map(|(t, d)| {
        (0..=*t).map(|i| (t - i) * i)
            .filter(|i| i > d)
            .count() as i64
        })
        .product::<i64>()
}

fn part2(txt: &str) -> i64 {
    let mut lines = txt.lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>().join(""));
    let time = lines.next().unwrap().split(":").skip(1).next().unwrap().parse::<i64>().unwrap();
    let dist = lines.next().unwrap().split(":").skip(1).next().unwrap().parse::<i64>().unwrap();

    (0..=time).map(|i| (time - i) * i)
        .filter(|i| *i > dist)
        .count() as i64
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
