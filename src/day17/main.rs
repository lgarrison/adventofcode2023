#![allow(non_snake_case)]

use std::{fs, collections::{VecDeque, BinaryHeap}, cmp::Reverse};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Grid {
    data: Vec<Vec<usize>>,
    N: usize,
}

impl Grid {
    fn new(txt: &str) -> Self {
        let data: Vec<Vec<usize>> = txt.lines().map(
            |line| line.chars().map(
                |c| c.to_digit(10).unwrap() as usize
            ).collect()
        ).collect();
        let N = data.len();
        Self { data, N }
    }

    fn get(&self, j: usize, i: usize) -> usize {
        self.data[j][i]
    }

    fn shortest(&self, minstraight: usize, maxstraight: usize) -> usize {
        let mut visited = vec![vec![vec![vec![false;maxstraight+1];5]; self.N]; self.N];
        let mut queue = BinaryHeap::new();
        queue.push((Reverse(0), 0, 0, Dir::Right, 0));
        queue.push((Reverse(0), 0, 0, Dir::Down, 0));
        while let Some((Reverse(d), j, i, dir, nstraight)) = queue.pop() {
            if (j, i) == (self.N-1, self.N-1) {
                return d;
            }
            if visited[j][i][dir as usize][nstraight] {
                // println!("Already visited ({}, {})", j, i);
                continue;
            }
            visited[j][i][dir as usize][nstraight] = true;
            if j > 0
                && !(dir == Dir::Up && nstraight >= maxstraight)
                && !(dir == Dir::Down)
                && !(dir != Dir::Up && nstraight < minstraight)
            {
                queue.push((Reverse(d+self.get(j-1, i)), j-1, i, Dir::Up, if dir == Dir::Up { nstraight+1 } else { 1 }));
            }
            if j < self.N-1
                && !(dir == Dir::Down && nstraight >= maxstraight)
                && !(dir == Dir::Up)
                && !(dir != Dir::Down && nstraight < minstraight)
            {
                queue.push((Reverse(d+self.get(j+1, i)), j+1, i, Dir::Down, if dir == Dir::Down { nstraight+1 } else { 1 }));
            }
            if i > 0
                && !(dir == Dir::Left && nstraight >= maxstraight)
                && !(dir == Dir::Right)
                && !(dir != Dir::Left && nstraight < minstraight)
            {
                queue.push((Reverse(d+self.get(j, i-1)), j, i-1, Dir::Left, if dir == Dir::Left { nstraight+1 } else { 1 }));
            }
            if i < self.N-1
                && !(dir == Dir::Right && nstraight >= maxstraight)
                && !(dir == Dir::Left)
                && !(dir != Dir::Right && nstraight < minstraight)
            {
                queue.push((Reverse(d+self.get(j, i+1)), j, i+1, Dir::Right, if dir == Dir::Right { nstraight+1 } else { 1 }));
            }
        }
        panic!("No path found");
    }
}

fn part1(txt: &str) -> usize {
    let grid = Grid::new(txt);
    grid.shortest(1, 3)
}

fn part2(txt: &str) -> usize {
    let grid = Grid::new(txt);
    grid.shortest(4, 10)
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
