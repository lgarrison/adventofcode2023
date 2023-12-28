#![allow(non_snake_case)]

use std::fs;

type Pos = [i64; 3];
type Vel = [i64; 3];

type FPos2 = [f64; 2];

#[derive(Debug, Clone, Copy)]
struct Rock {
    pos: Pos,
    vel: Vel,
}

fn intersection2d(r1: &Rock, r2: &Rock) -> Option<FPos2> {
    let p1 = [r1.pos[0] as f64, r1.pos[1] as f64];
    let p2 = [r2.pos[0] as f64, r2.pos[1] as f64];
    let v1 = [r1.vel[0] as f64, r1.vel[1] as f64];
    let v2 = [r2.vel[0] as f64, r2.vel[1] as f64];
    let t1 = (v2[0] * (p1[1] - p2[1]) - v2[1] * (p1[0] - p2[0])) / 
        (v2[1] * v1[0] - v2[0] * v1[1]);
    let t2 = (v1[0] * (p2[1] - p1[1]) - v1[1] * (p2[0] - p1[0])) /
        (v1[1] * v2[0] - v1[0] * v2[1]);
    if t1 >= 0f64 && t2 >= 0f64 {
        // println!("{} {} {} {}", p1[0], p1[1], v1[0], v1[1]);
        return Some([p1[0] + t1 * v1[0], p1[1] + t1 * v1[1]]);
    } else {
        None
    }
}

fn part1(txt: &str) -> usize {

    let mut rocks: Vec<Rock> = Vec::new();
    for line in txt.lines() {
        let parts = line.split([',','@'])
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        rocks.push( Rock {
            pos: [parts[0], parts[1], parts[2]],
            vel: [parts[3], parts[4], parts[5]],
        });
    }

    // let min = 7f64; let max = 27f64;
    let min = 200000000000000f64; let max = 400000000000000f64;

    let nintersect = rocks.iter()
        .enumerate()
        .map(|(i, r1)| {
            rocks[i+1..].iter()
                .filter_map(|r2| intersection2d(r1, r2))
                .filter(|x| x[0] >= min && x[0] <= max && x[1] >= min && x[1] <= max)
                .count()
        })
        .sum::<usize>();

    nintersect
}

fn part2(txt: &str) -> i64 {
    let mut rocks: Vec<Rock> = Vec::new();
    for line in txt.lines() {
        let parts = line.split([',','@'])
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        rocks.push( Rock {
            pos: [parts[0], parts[1], parts[2]],
            vel: [parts[3], parts[4], parts[5]],
        });
    }

    0
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
