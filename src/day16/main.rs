#![allow(non_snake_case)]

use core::panic;
use std::fs;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Mirror {
    Empty,
    VSplit,
    HSplit,
    Slash,
    Backslash,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Mirror {
    fn from_char(c: char) -> Mirror {
        match c {
            '.' => Mirror::Empty,
            '|' => Mirror::VSplit,
            '-' => Mirror::HSplit,
            '/' => Mirror::Slash,
            '\\' => Mirror::Backslash,
            _ => panic!("Unknown mirror: {}", c),
        }
    }

    fn reflect(&self, beamdir: Dir) -> Vec<Dir> {
        match self {
            Mirror::Empty => vec![beamdir],
            Mirror::VSplit => match beamdir {
                Dir::Up | Dir::Down => vec![beamdir],
                Dir::Left | Dir::Right => vec![Dir::Up, Dir::Down],
                Dir::None => panic!("Beamdir is None"),
            },
            Mirror::HSplit => match beamdir {
                Dir::Up | Dir::Down => vec![Dir::Left, Dir::Right],
                Dir::Left | Dir::Right => vec![beamdir],
                Dir::None => panic!("Beamdir is None"),
            },
            Mirror::Slash => match beamdir {
                Dir::Up => vec![Dir::Right],
                Dir::Down => vec![Dir::Left],
                Dir::Left => vec![Dir::Down],
                Dir::Right => vec![Dir::Up],
                Dir::None => panic!("Beamdir is None"),
            },
            Mirror::Backslash => match beamdir {
                Dir::Up => vec![Dir::Left],
                Dir::Down => vec![Dir::Right],
                Dir::Left => vec![Dir::Up],
                Dir::Right => vec![Dir::Down],
                Dir::None => panic!("Beamdir is None"),
            },
        }
    }
}

struct Grid<T> {
    grid: Vec<Vec<T>>,
    N: usize,
}

impl Grid<Mirror> {
    fn from_str(txt: &str) -> Grid<Mirror> {
        Grid {
            grid: txt.lines().map(|line| line.chars().map(|c| Mirror::from_char(c)).collect()).collect(),
            N: txt.lines().count(),
        }
    }

    fn get(&self, j: usize, i: usize) -> Mirror {
        self.grid[j][i]
    }

    // Starting from the top left and heading to the right,
    // follow the beam until it exits the grid.
    // Return a grid of the spaces the beam passes through.
    fn follow_beam(&self, j: usize, i: usize, dir: Dir) -> Grid<bool> {
        let mut grid = vec![vec![Dir::None; self.N]; self.N];

        let mut wavefronts = vec![(j, i, dir)];
        while let Some((j, i, beamdir)) = wavefronts.pop() {
            grid[j][i] = beamdir;

            let mirror = self.get(j, i);
            let newdirs = mirror.reflect(beamdir);
            for newdir in newdirs {
                let (dj, di) = match newdir {
                    Dir::Up => (-1, 0),
                    Dir::Down => (1, 0),
                    Dir::Left => (0, -1),
                    Dir::Right => (0, 1),
                    Dir::None => panic!("Beamdir is None"),
                };
                let newj = j as i64 + dj;
                let newi = i as i64 + di;
                if newj < 0 || newj >= self.N as i64 || newi < 0 || newi >= self.N as i64 {
                    continue;
                }
                let newj = newj as usize;
                let newi = newi as usize;
                if grid[newj][newi] != newdir {
                    wavefronts.push((newj, newi, newdir));
                }
            }
        }
        Grid {
            grid: grid.iter().map(|row| row.iter().map(|dir| *dir != Dir::None).collect()).collect(),
            N: self.N,
        }
    }
}

impl Grid<bool> {
    fn sum(&self) -> usize {
        self.grid.iter().map(|row| row.iter().map(|b| if *b { 1 } else { 0 }).sum::<usize>()).sum()
    }
}
        


fn part1(txt: &str) -> usize {
    let grid = Grid::from_str(txt);
    let beamgrid = grid.follow_beam(0, 0, Dir::Right);
    beamgrid.sum()
}

fn part2(txt: &str) -> usize {
    let grid = Grid::from_str(txt);

    let mut max: usize = 0;
    for i in 0..grid.N {
        let beamgrid = grid.follow_beam(0, i, Dir::Down);
        if beamgrid.sum() > max {
            max = beamgrid.sum();
        }
    }
    for j in 0..grid.N {
        let beamgrid = grid.follow_beam(j, 0, Dir::Right);
        if beamgrid.sum() > max {
            max = beamgrid.sum();
        }
    }
    for i in 0..grid.N {
        let beamgrid = grid.follow_beam(grid.N - 1, i, Dir::Up);
        if beamgrid.sum() > max {
            max = beamgrid.sum();
        }
    }
    for j in 0..grid.N {
        let beamgrid = grid.follow_beam(j, grid.N - 1, Dir::Left);
        if beamgrid.sum() > max {
            max = beamgrid.sum();
        }
    }
    max
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
