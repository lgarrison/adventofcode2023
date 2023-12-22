#![allow(non_snake_case)]

use std::{fs, collections::{HashMap, HashSet, VecDeque, BinaryHeap}, cmp::Reverse};

type Pos2 = [usize;2];
type Pos3 = [usize;3];
type PID = usize;

# [derive(Debug)]
struct Piece {
    below: HashSet<PID>,
    above: HashSet<PID>,
    zmin: usize,
}

# [derive(Debug)]
struct Board {
    pieces: Vec<Piece>,
    highest: HashMap<Pos2,(usize,PID)>,
}

impl Board {
    fn new() -> Board {
        Board {
            pieces: vec![],
            highest: HashMap::new(),
        }
    }

    fn drop(&mut self, start: Pos3, end: Pos3) {
        let new_pid: PID = self.pieces.len();
        let [x,y,z] = start;
        let [x2,y2,z2] = end;
        let mut min: Pos3 = [x.min(x2), y.min(y2), z.min(z2)];
        let mut max: Pos3 = [x.max(x2), y.max(y2), z.max(z2)];
        let zlen = max[2] - min[2] + 1;
        
        let mut zdrop = 1;
        for i in min[0]..=max[0] {
            for j in min[1]..=max[1] {
                zdrop = zdrop.max(self.highest.get(&[i,j]).unwrap_or(&(0,0)).0 + 1);
            }
        }

        min[2] = zdrop;
        max[2] = zdrop + zlen - 1;

        let mut below: HashSet<PID> = HashSet::new();
        for i in min[0]..=max[0] {
            for j in min[1]..=max[1] {
                if let Some(&(height,pid)) = self.highest.get(&[i,j]) {
                    if height == min[2] - 1 {
                        below.insert(pid);
                    }
                }
                self.highest.insert([i,j], (max[2], new_pid));
            }
        }

        for pid in below.iter() {
            self.pieces[*pid].above.insert(new_pid);
        }

        self.pieces.push(
            Piece {
                below,
                above: HashSet::new(),
                zmin: min[2],
            }
        );
    }

    fn count_safe(&self) -> usize {
        self.pieces.iter()
            .filter(|p| p.above.iter().all(|f| self.pieces[*f].below.len() > 1))
            .count()
    }

    fn wouldfall(&self, pid: PID) -> usize {
        let mut supporting: HashSet<PID> = HashSet::from_iter(vec![pid]);
        let mut queue: BinaryHeap<(Reverse<usize>,PID)> = BinaryHeap::from_iter(vec![(Reverse(self.pieces[pid].zmin), pid)]);
        while let Some((_,p)) = queue.pop() {
            for &p2 in self.pieces[p].above.iter() {
                if self.pieces[p2].below.iter().all(|b| supporting.contains(b)) {
                    supporting.insert(p2);
                }
                queue.push((Reverse(self.pieces[p2].zmin), p2));
            }
        }
        supporting.len() - 1
    }
}

fn part1(txt: &str) -> usize {
    let mut board = Board::new();
    for line in txt.lines() {
        let parts = line.split('~').collect::<Vec<&str>>();
        let _start = parts[0].split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let start: Pos3 = [_start[0], _start[1], _start[2]];
        let _end = parts[1].split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let end: Pos3 = [_end[0], _end[1], _end[2]];

        board.drop(start, end);
    }
    // println!("{:?}", board);
    board.count_safe() as usize
}

fn part2(txt: &str) -> usize {
    let mut board = Board::new();
    let mut pairs = txt.lines().map(|line| {
        let parts = line.split('~').collect::<Vec<&str>>();
        let _start = parts[0].split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let start: Pos3 = [_start[0], _start[1], _start[2]];
        let _end = parts[1].split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let end: Pos3 = [_end[0], _end[1], _end[2]];

        [start,end]
    }).collect::<Vec<[Pos3;2]>>();

    // sort pairs by z min:
    pairs.sort_by(|a,b| a[0][2].min(a[1][2])
        .cmp(
            &b[0][2].min(b[1][2])
        ));
    
    for p in pairs.iter() {
        board.drop(p[0], p[1]);
    }
    
    // for i in 0..board.pieces.len() {
    //     println!("{}: {:?}", i, board.wouldfall(i));
    // }
    (0..board.pieces.len()).map(|p| board.wouldfall(p)).sum()
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
