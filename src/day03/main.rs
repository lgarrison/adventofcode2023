#![allow(non_snake_case)]

use std::{fs, collections::{HashMap, HashSet}};

#[derive(Debug)]
struct Grid {
    grid: Vec<i64>,
    Nx: usize,
    Ny: usize,
    valmap: HashMap<i64, i64>,
}

const DOT: i64 = 0;
const SYM: i64 = 1;
const GEAR: i64 = 2;

impl Grid {
    fn from_txt(txt: &str) -> Self {
        let mut grid = Self {
            Nx: txt.lines().count(),
            Ny: txt.lines().next().unwrap().len(),
            grid: vec![],
            valmap: HashMap::new(),
        };
        grid.grid = vec![DOT; grid.Nx * grid.Ny];

        let mut key = GEAR + 1;
        for (j,line) in txt.lines().enumerate() {
            let mut i = 0;
            let chars: Vec<char> = line.chars().collect();
            while i < line.len() {
                let c = chars[i];
                if c.is_digit(10) {
                    let len: usize = chars[i..].iter().position(|c| !c.is_digit(10)).unwrap_or(line.len() - i);
                    grid.grid[(grid.Nx*j + i)..(grid.Nx*j + i + len)].fill(key);
                    grid.valmap.insert(key, chars[i..(i+len)].iter().collect::<String>().parse::<i64>().unwrap());
                    key += 1;
                    i += len;
                }
                else if c == '*' {
                    grid.grid[grid.Nx*j + i] = GEAR;
                    i += 1;
                }
                else if c != '.' {
                    grid.grid[grid.Nx*j + i] = SYM;
                    i += 1;
                }
                else {
                    i += 1;
                }
            }
        }
        grid
    }

    fn get(&self, i: i64, j: i64) -> Option<i64> {
        if i < 0 || j < 0 || i >= self.Nx as i64 || j >= self.Ny as i64 {
            None
        }
        else {
            Some(self.grid[self.Nx*j as usize + i as usize])
        }
    }

    // neighbor part numbers
    fn neighbors(&mut self, i: usize, j: usize) -> Vec<i64> {
        let mut keyset = HashSet::new();
        for dj in -1..=1 {
            for di in -1..=1 {
                if di == 0 && dj == 0 {
                    continue;
                }
                if let Some(k) = self.get(i as i64 + di, j as i64 + dj) {
                    if k > GEAR {
                        keyset.insert(k);
                    }
                }
            }
        }
        let neighbors = keyset.iter()
                        .map(|k| *self.valmap.get(k).unwrap())
                        .collect::<Vec<i64>>();
        neighbors
    }
}

fn part1(txt: &str) -> i64 {
    let mut grid = Grid::from_txt(txt);
    // println!("{:?}", grid);

    let mut sum = 0;
    for j in 0..grid.Ny {
        for i in 0..grid.Nx {
            if let SYM | GEAR = grid.grid[grid.Nx*j + i] {
                sum += grid.neighbors(i, j).iter().sum::<i64>();
            }
        }
    }

    sum
}

fn part2(txt: &str) -> i64 {
    let mut grid = Grid::from_txt(txt);
    // println!("{:?}", grid);

    let mut sum = 0;
    for j in 0..grid.Ny {
        for i in 0..grid.Nx {
            if grid.grid[grid.Nx*j + i] == GEAR {
                let neigh = grid.neighbors(i, j);
                // println!("{:?}", neigh);
                if neigh.len() == 2 {
                    sum += neigh[0] * neigh[1];
                }
            }
        }
    }

    sum
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
