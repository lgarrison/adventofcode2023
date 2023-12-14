#![allow(non_snake_case)]

use core::panic;
use std::{fs, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Square,
    Round,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Rock {
    fn from_char(c: char) -> Rock {
        match c {
            '#' => Rock::Square,
            'O' => Rock::Round,
            '.' => Rock::Empty,
            _ => panic!("Unknown rock type"),
        }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Grid {
    data: Vec<Vec<Rock>>,
    N: usize,
}

impl Grid {
    fn from_str(txt: &str) -> Grid {
        let mut data = Vec::new();
        for line in txt.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(Rock::from_char(c));
            }
            data.push(row);
        }
        let N = data.len();
        Grid {
            data,
            N,
        }
    }

    fn get(&self, j: usize, i: usize) -> Rock {
        self.data[j][i]
    }

    fn set(&mut self, j: usize, i: usize, rock: Rock) {
        self.data[j][i] = rock;
    }

    fn print(&self) {
        for row in &self.data {
            for r in row {
                match r {
                    Rock::Square => print!("#"),
                    Rock::Round => print!("O"),
                    Rock::Empty => print!("."),
                }
            }
            println!();
        }
        println!();
    }

    fn roll(&mut self, dir: Dir) {
        let irange: Vec<usize> = match dir {
            Dir::North => (0..self.N).collect(),
            Dir::South => (0..self.N).collect(),
            Dir::East => (0..self.N).collect(),
            Dir::West => (0..self.N).collect(),
        };
        let jrange: Vec<usize> = match dir {
            Dir::North => (0..self.N).collect(),
            Dir::South => (0..self.N).rev().collect(),
            Dir::East => (0..self.N).rev().collect(),
            Dir::West => (0..self.N).collect(),
        };

        for &i in irange.iter() {
            let mut fallpos = jrange[0];
            for &j in jrange.iter() {
                let rock = match dir {
                    Dir::North | Dir::South => self.get(j,i),
                    Dir::East | Dir::West => self.get(i,j),
                };
                match rock {
                    Rock::Round => {
                        match dir {
                            Dir::North | Dir::South => {
                                self.set(j,i,Rock::Empty);
                                self.set(fallpos,i,Rock::Round);
                            }
                            Dir::East | Dir::West => {
                                self.set(i,j,Rock::Empty);
                                self.set(i,fallpos,Rock::Round);
                            }
                        }
                        fallpos = match dir {
                            Dir::North | Dir::West => fallpos + 1,
                            Dir::South | Dir::East => if fallpos > 0 { fallpos - 1 } else { fallpos },
                        };
                    },
                    Rock::Square => {
                        fallpos = match dir {
                            Dir::North | Dir::West => j + 1,
                            Dir::South | Dir::East => if j > 0 { j - 1 } else { j },
                        };
                    },
                    _ => (),
                }
            }
        }
    }

    fn load(&self) -> i64 {
        self.data.iter().enumerate().map(|(j,row)| {
           row.iter().filter(|r| **r == Rock::Round).count()*(self.N - j)
        }).sum::<usize>() as i64
    }

    fn p2(&mut self) -> i64 {
        let mut seen: HashMap<Grid,usize> = HashMap::new();
        for i in 0..1000000000 {
            for &dir in [Dir::North, Dir::West, Dir::South, Dir::East].iter() {
                self.roll(dir);
            }
            if seen.contains_key(&self.clone()) {
                let prev_iter = *seen.get(&self).unwrap();
                let cycle_len = i - prev_iter;
                let remaining_tilts = 1000000000 - (i+1);
                let rem = remaining_tilts % cycle_len;
                println!("Cycle len: {}, remaining: {}", cycle_len, rem);
                if rem == 0 {
                    return self.load();
                }
            } else {
                seen.insert(self.clone(),i);
            }
        }
        0
    }
}

fn part1(txt: &str) -> i64 {
    let mut grid = Grid::from_str(txt);
    grid.roll(Dir::North);
    grid.load()
}

fn part2(txt: &str) -> i64 {
    let mut grid = Grid::from_str(txt);    
    grid.p2()
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
