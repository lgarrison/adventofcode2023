#![allow(non_snake_case)]

use std::fs;

struct Grid {
    data: Vec<Vec<char>>,
    loop_path: Vec<(usize, usize)>,
    S: (usize, usize),
    N: usize,
}

impl Grid { 
    fn new(txt: &str) -> Grid {
        let data: Vec<Vec<char>> = txt.lines().map(|line| line.chars().collect()).collect();
        let N = data.len();

        let mut S = (0,0);
        for j in 0..data.len() {
            for i in 0..data[j].len() {
                match data[j][i] {
                    'S' => { S = (j,i); break; },
                    _ => (),
                }
            }
        }
        Grid { data,
            S,
            N,
            loop_path: Vec::new(),
        }
    }

    fn get(&self, pos: (usize, usize)) -> char {
        self.data[pos.0][pos.1]
    }

    fn loop_circ(&self) -> (usize, Vec<(usize, usize)>) {
        let mut prev = self.S;

        // prime the pos
        let mut pos: (usize, usize) = 
            if prev.0 > 0 && ['|', 'F', '7'].contains(&self.data[prev.0-1][prev.1]) {
                // up
                (prev.0-1, prev.1)
            } else if prev.0 < self.N-1 && ['|', 'J', 'L'].contains(&self.data[prev.0+1][prev.1]) {
                // down
                (prev.0+1, prev.1)
            } else if prev.1 > 0 && ['-', 'F', 'L'].contains(&self.data[prev.0][prev.1-1]) {
                // left
                (prev.0, prev.1-1)
            } else if prev.1 < self.N-1 && ['-', 'J', '7'].contains(&self.data[prev.0][prev.1+1]) {
                // right
                (prev.0, prev.1+1)
            } else {
                panic!("No start direction found!");
            };
            
        let mut pathlen = 1;
        let mut loop_path: Vec<(usize, usize)> = vec![self.S];
        while pos != self.S {
            let next = match self.get(pos) {
                '|' => {
                    if (pos.0 + 1, pos.1) == prev {
                        (pos.0 - 1, pos.1)
                    } else {
                        (pos.0 + 1, pos.1)
                    }
                },
                '-' => {
                    if (pos.0, pos.1 + 1) == prev {
                        (pos.0, pos.1 - 1)
                    } else {
                        (pos.0, pos.1 + 1)
                    }
                },
                'L' => {
                    if (pos.0 - 1, pos.1) == prev {
                        (pos.0, pos.1 + 1)
                    } else {
                        (pos.0 - 1, pos.1)
                    }
                },
                '7' => {
                    if (pos.0, pos.1 - 1) == prev {
                        (pos.0 + 1, pos.1)
                    } else {
                        (pos.0, pos.1 - 1)
                    }
                },
                'F' => {
                    if (pos.0 + 1, pos.1) == prev {
                        (pos.0, pos.1 + 1)
                    } else {
                        (pos.0 + 1, pos.1)
                    }
                },
                'J' => {
                    if (pos.0, pos.1 - 1) == prev {
                        (pos.0 - 1, pos.1)
                    } else {
                        (pos.0, pos.1 - 1)
                    }
                },
                _ => panic!("Unknown char: {}", self.get(pos)),
            };
            prev = pos;
            pos = next;
            if ['L', 'F', '7', 'J', 'S'].contains(&self.get(pos)) {
                loop_path.push(pos);
            }
            pathlen += 1;
        }

        (pathlen, loop_path)
    }

    fn shoelace(&self) -> i64 {
        let mut sum = 0i64;
        let L = self.loop_path.len() as i64;
        for i in 0i64..L {
            sum += self.loop_path[i as usize].0 as i64 * (
                self.loop_path[(i+1).rem_euclid(L) as usize].1 as i64
                - self.loop_path[(i-1).rem_euclid(L) as usize].1 as i64
            );
        }
        sum.abs()
    }
}

fn part1(txt: &str) -> i64 {
    let grid = Grid::new(txt);

    (grid.loop_circ().0 / 2) as i64
}

fn part2(txt: &str) -> i64 {
    let mut grid = Grid::new(txt);

    let p1: (usize, Vec<(usize, usize)>) = grid.loop_circ();
    let circ = p1.0;
    grid.loop_path = p1.1;
    (grid.shoelace() - circ as i64 + 1) / 2 + 1
}

fn main() {
    let dayX = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + dayX + "/input.txt";
    // let path = String::from(root) + "/src/" + dayX + "/test1.txt";
    // let path = String::from(root) + "/src/" + dayX + "/test2.txt";
    // let path = String::from(root) + "/src/" + dayX + "/test3.txt";
    // let path = String::from(root) + "/src/" + dayX + "/test4.txt";
    // let path = String::from(root) + "/src/" + dayX + "/test5.txt";
    // let path = String::from(root) + "/src/" + dayX + "/mytest.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", dayX);
    println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}
