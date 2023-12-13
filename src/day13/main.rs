#![allow(non_snake_case)]

use std::fs;

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<bool>>,
    Nx: usize,
    Ny: usize,
}

impl Grid {
    fn new(txt: &str) -> Grid {
        let mut data = Vec::new();
        let mut Nx = 0;
        let mut Ny = 0;
        for line in txt.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                match c {
                    '.' => row.push(false),
                    '#' => row.push(true),
                    _ => panic!("Unknown character"),
                }
            }
            Nx = row.len();
            Ny += 1;
            data.push(row);
        }
        Grid { data, Nx, Ny }
    }

    fn score_reflections(&self) -> usize {
        let mut count = 0;
        // horizontal reflections
        for i in 0..self.Nx-1 {
            if (0..i+1).all(|off|
                (0..self.Ny).all(|j| i+off+1 >= self.Nx || self.data[j][i-off] == self.data[j][i+off+1])
            ){
                count += i + 1;
                break;
            }
        }
        // println!("h count: {}", count);

        // vertical reflections
        for j in 0..self.Ny-1 {
            if (0..j+1).all(|off|
                (0..self.Nx).all(|i| j+off+1 >= self.Ny || self.data[j-off][i] == self.data[j+off+1][i])
            ){
                count += 100*(j + 1);
                break;
            }
        }
        // println!("h+v count: {}", count);

        count
    }

    fn score_reflections_p2(&self) -> usize {
        let mut count = 0;
        // horizontal reflections
        for i in 0..self.Nx-1 {
            if (0..i+1).map(|off|
                (0..self.Ny).filter(|&j| i+off+1 < self.Nx && self.data[j][i-off] != self.data[j][i+off+1])
                .count()
            ).sum::<usize>() == 1 {
                count += i + 1;
                break;
            }
        }
        println!("h count: {}", count);

        // vertical reflections
        for j in 0..self.Ny-1 {
            if (0..j+1).map(|off|
                (0..self.Nx).filter(|&i| j+off+1 < self.Ny && self.data[j-off][i] != self.data[j+off+1][i])
                .count()
            ).sum::<usize>() == 1 {
                count += 100*(j + 1);
                break;
            }
        }

        println!("h+v count: {}", count);

        count
    }
}

fn part1(txt: &str) -> i64 {
    let grids = txt.split("\n\n")
        .map(|g| Grid::new(g));
    // println!("{:?}", grid);
    grids.map(|g| g.score_reflections()).sum::<usize>() as i64
}

fn part2(txt: &str) -> i64 {
    let grids = txt.split("\n\n")
        .map(|g| Grid::new(g));
    // println!("{:?}", grid);
    grids.map(|g| g.score_reflections_p2()).sum::<usize>() as i64
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
