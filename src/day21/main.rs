#![allow(non_snake_case)]

use std::{fs, collections::VecDeque};

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<bool>>,
    N: usize,
    start: (usize, usize),
}

impl Grid {
    fn new(txt: &str) -> Self {
        let mut start = (0, 0);
        let data: Vec<Vec<bool>> = txt
            .lines().enumerate()
            .map(|(j,line)| {
                line.chars().enumerate()
                    .map(|(i,c)| {
                        if c == 'S' { start = (j,i) };
                        if c == '#' { true } else { false }
                    }).collect()
            }).collect();
        let N = data.len();
        Self { data, N, start }
    }

    fn neighbors(&self, j: usize, i: usize, b: usize, a: usize) -> Vec<(usize,usize,usize,usize)> {
        let mut neighbors = vec![];
        if j > 0 {
            if !self.data[j-1][i] { neighbors.push((j-1,i,b,a)) };
        } else {
            if !self.data[self.N-1][i] { neighbors.push((self.N-1,i,b-1,a)) };
        }
        if i > 0 {
            if !self.data[j][i-1] { neighbors.push((j,i-1,b,a)) };
        } else {
            if !self.data[j][self.N-1] { neighbors.push((j,self.N-1,b,a-1)) };
        }
        if j < self.N-1 {
            if !self.data[j+1][i] { neighbors.push((j+1,i,b,a)) };
        } else {
            if !self.data[0][i] { neighbors.push((0,i,b+1,a)) };
        }
        if i < self.N-1 {
            if !self.data[j][i+1] { neighbors.push((j,i+1,b,a)) };
        } else {
            if !self.data[j][0] { neighbors.push((j,0,b,a+1)) };
        }

        // if j > 0 && !self.data[j-1][i] { neighbors.push((j-1,i)) };
        // if i > 0 && !self.data[j][i-1] { neighbors.push((j,i-1)) };
        // if j < self.N-1 && !self.data[j+1][i] { neighbors.push((j+1,i)) };
        // if i < self.N-1 && !self.data[j][i+1] { neighbors.push((j,i+1)) };
        neighbors
    }

    fn reachable(&self, nstep: usize) -> usize {
        let mut queue: VecDeque<((usize,usize),(usize,usize),usize)> = VecDeque::new();
        let mut even: Vec<Vec<Vec<Vec<bool>>>> = vec![vec![vec![vec![false; self.N]; self.N]; 7]; 7];

        queue.push_back((self.start, (3,3), 0));

        while let Some(((j,i),(b,a),step)) = queue.pop_front() {
            if step > nstep { continue; }
            if even[b][a][j][i] { continue; }
            if step % 2 == (nstep % 2) { even[b][a][j][i] = true; }
            for (jj,ii, bb, aa) in self.neighbors(j,i,b,a) {
                queue.push_back(((jj,ii),(bb,aa),step+1));
            }
        }
        // even.iter().map(|row| row.iter().filter(|&&x| x).count()).sum()
        even.iter().flatten().flatten().flatten().filter(|&&x| x).count()
    }
}

fn part1(txt: &str) -> usize {
    let grid = Grid::new(txt);
    // println!("{:?}", grid);
    grid.reachable(64)
}

fn part2(txt: &str) -> usize {
    let grid = Grid::new(txt);
    let S = 26501365;
    // let S = 50;
    // println!("{:?}", grid);
    let n1 = grid.reachable(S % grid.N) as i64;
    let n2 = grid.reachable((S % grid.N) + grid.N) as i64;
    let n3 = grid.reachable((S % grid.N) + 2*grid.N) as i64;

    println!("{} {} {}", n1, n2, n3);
    // println!("{} {}", n1, n2);

    let M = (S / grid.N) as i64;
    (n1 + (n2-n1)*M + (n3-2*n2+n1)*(M*(M-1)/2)) as usize
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
