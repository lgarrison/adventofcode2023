#![allow(non_snake_case)]

use std::fs;



fn part1(txt: &str) -> usize {
    let maxcubes: [i64; 3] = [12, 13, 14];  // red, green, blue

    txt.lines().enumerate().filter_map(
        |(i, line)| {
            let tokens: Vec<&str> = line.split([' ', ',', ';'])
                                        .filter(|w| w.len() > 0)
                                        .skip(2)
                                        .collect();
            tokens.chunks(2).all(|pair|
                pair[0].parse::<i64>().unwrap() <= maxcubes[
                    match pair[1] {
                        "red" => 0,
                        "green" => 1,
                        "blue" => 2,
                        _ => panic!("Unknown color"),
                    }
                ]
            ).then_some(i+1)
        }
    ).sum()
}

fn part2(txt: &str) -> i64 {
    txt.lines().map(
        |line| {
            let tokens: Vec<&str> = line.split([' ', ',', ';'])
                                        .filter(|w| w.len() > 0)
                                        .skip(2)
                                        .collect();
            let mut maxcubes: [i64; 3] = [0,0,0];  // red, green, blue
            for pair in tokens.chunks(2) {
                let cid = match pair[1] {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => panic!("Unknown color"),
                };
                let count = pair[0].parse::<i64>().unwrap();
                if count > maxcubes[cid] {
                    maxcubes[cid] = count;
                }
            }
            maxcubes.iter().product::<i64>()
        }
    ).sum()
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
