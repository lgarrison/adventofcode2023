#![allow(non_snake_case)]

use std::fs;

fn shoelace2(vertices: &Vec<(i64, i64)>) -> i64 {
    let mut A = 0;
    let n = vertices.len();
    for i in 0..n {
        let j = (i + 1) % n;
        A += (vertices[i].1 + vertices[j].1) * (vertices[i].0 - vertices[j].0);
    }
    A.abs()
}

fn part1(txt: &str) -> i64 {
    let vertices = txt
        .lines()
        .scan((0,0), |state, line| {
            let mut tokens = line.split_whitespace();
            let dir = tokens.next().unwrap();
            let n = tokens.next().unwrap().parse::<i64>().unwrap();
            match dir {
                "R" => state.0 += n,
                "L" => state.0 -= n,
                "U" => state.1 += n,
                "D" => state.1 -= n,
                _ => panic!("Unknown direction"),
            }
            Some(*state)
        }).collect();
    let A2 = shoelace2(&vertices);
    let cyc: Vec<&(i64, i64)> = vertices.iter().cycle().take(vertices.len()+1).collect();
    let b= cyc.windows(2).fold(0, |acc, pair|
        acc + (pair[0].1 - pair[1].1).abs() + (pair[0].0 - pair[1].0).abs()
    );
    (A2 - b)/2 + 1 + b
}

fn part2(txt: &str) -> i64 {
    let vertices = txt
        .lines()
        .scan((0,0), |state, line| {
            let mut tokens = line.split('#');
            let hex = &tokens.skip(1).next().unwrap()[..6];
            let n = i64::from_str_radix(&hex[0..5], 16).unwrap();
            let dir = &hex[5..6].parse::<i64>().unwrap();
            match dir {
                0 => state.0 += n,
                1 => state.1 += n,
                2 => state.0 -= n,
                3 => state.1 -= n,
                _ => panic!("Unknown direction"),
            }
            Some(*state)
        }).collect();
    let A2 = shoelace2(&vertices);
    let cyc: Vec<&(i64, i64)> = vertices.iter().cycle().take(vertices.len()+1).collect();
    let b= cyc.windows(2).fold(0, |acc, pair|
        acc + (pair[0].1 - pair[1].1).abs() + (pair[0].0 - pair[1].0).abs()
    );
    (A2 - b)/2 + 1 + b
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
