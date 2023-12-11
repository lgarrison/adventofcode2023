#![allow(non_snake_case)]

use std::fs;

struct Universe {
    gals: Vec<(usize,usize)>,
}

impl Universe {
    fn new(txt: &str, expansion: usize) -> Universe {
        let mut gals: Vec<(usize,usize)> = txt.lines()
            .enumerate()
            .flat_map(|(j, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(i, c)| if c == '#' { Some ((j, i)) } else { None })
                    .collect::<Vec<(usize,usize)>>()
            }).collect();

        let Ngrid = txt.lines().next().unwrap().len();

        let mut colcounts = vec![0; Ngrid];
        let mut rowcounts = vec![0; Ngrid];
        for g in gals.iter() {
            colcounts[g.1] += 1;
            rowcounts[g.0] += 1;
        }
        let colempty = colcounts.iter().map(|c| if *c > 0 { 0 } else { expansion-1 }).collect::<Vec<usize>>();
        let rowempty = rowcounts.iter().map(|c| if *c > 0 { 0 } else { expansion-1 }).collect::<Vec<usize>>();

        // prefix sum
        let coloffsets = colempty.iter().scan(0, |acc, &x| { *acc += x; Some(*acc) }).collect::<Vec<usize>>();
        let rowoffsets = rowempty.iter().scan(0, |acc, &x| { *acc += x; Some(*acc) }).collect::<Vec<usize>>();

        // println!("coloffsets: {:?}", coloffsets);
        // println!("rowoffsets: {:?}", rowoffsets);
        for g in gals.iter_mut() {
            g.1 += coloffsets[g.1];
            g.0 += rowoffsets[g.0];
        }

        Universe {
            gals,
        }
    }

    fn pair_dist(&self, j: usize, i: usize) -> usize {
        let res = ((self.gals[j].0 as i64 - self.gals[i].0 as i64).abs() +
        (self.gals[j].1 as i64 - self.gals[i].1 as i64).abs())
            as usize;
        // println!("{} {} {}", j, i, res);
        res
    }
}

fn part1(txt: &str) -> usize {
    let u = Universe::new(txt, 2);
    let N = u.gals.len();
    (0..N).map(|j| {
        (j+1..N).map(|i| u.pair_dist(j, i))
            .sum::<usize>()
    }).sum()
}

fn part2(txt: &str) -> usize {
    let u = Universe::new(txt, 1000000);
    let N = u.gals.len();
    (0..N).map(|j| {
        (j+1..N).map(|i| u.pair_dist(j, i))
            .sum::<usize>()
    }).sum()
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
