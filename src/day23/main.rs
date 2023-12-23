#![allow(non_snake_case)]

use std::{fs, collections::{HashSet, HashMap}};

type Pos = [usize;2];

#[derive(Debug)]
struct Grid {
    data: Vec<char>,
    N: usize,
    start: Pos,
    end: Pos,
    p2: bool,
}

impl Grid {
    fn new(txt: &str, p2: bool) -> Self {
        let lines = txt.lines().collect::<Vec<&str>>();
        let data = lines.join("").chars().collect::<Vec<char>>();
        let N = lines.len();
        let start = [0,lines[0].chars().position(|c| c == '.').unwrap()];
        let end = [N-1,lines[N-1].chars().position(|c| c == '.').unwrap()];
        Self { data, N, start, end, p2 }
    }

    fn get(&self, j: usize, i: usize) -> char {
        self.data[j * self.N + i]
    }

    fn get_pos(&self, pos: Pos) -> char {
        self.get(pos[0], pos[1])
    }

    fn find_longest_path(&self, pos: Pos, mut visited: HashSet<Pos>) -> usize {
        if pos == self.end {
            return 0;
        }

        let thischar = self.get_pos(pos);
        visited.insert(pos);

        let mut maxlen = 0;
        for dir in [[0i64,1],[0,-1],[1,0],[-1,0]].iter() {
            if !self.p2 {
                match thischar {
                    '>' => if dir != &[0,1] { continue; },
                    '<' => if dir != &[0,-1] { continue; },
                    'v' => if dir != &[1,0] { continue; },
                    '^' => if dir != &[-1,0] { continue; },
                    _ => (),
                };
            }
            let _newpos = [pos[0] as i64 + dir[0], pos[1] as i64 + dir[1]];
            if _newpos[0] < 0 || _newpos[0] >= self.N as i64 || _newpos[1] < 0 || _newpos[1] >= self.N as i64 {
                continue;
            }
            let newpos = [_newpos[0] as usize, _newpos[1] as usize];
            let newchar = self.get_pos(newpos);
            if newchar == '#' || visited.contains(&newpos) {
                continue;
            }
            let pathlen = self.find_longest_path(newpos, visited.clone()) + 1;
            maxlen = maxlen.max(pathlen);
        }
        return maxlen;
    }
}

fn part1(txt: &str) -> usize {
    // let grid = Grid::new(txt, false);
    // grid.find_longest_path(grid.start, HashSet::new())

    let mut grid = SparseGrid::new(txt, false);
    // println!("{:?}", grid);
    // grid.print_edges();
    grid.contract();
    // grid.print_edges();
    grid.find_longest_path(grid.start, HashSet::new()).unwrap()
}

// ------------------------------------------------------------------ //

#[derive(Debug)]
struct SparseGrid {
    edges: HashMap<usize,HashMap<usize,usize>>,
    p2: bool,
    start: usize,
    end: usize,
    N: usize,
}

impl SparseGrid {
    fn new(txt: &str, p2: bool) -> Self {
        let g = Grid::new(txt, true);

        let mut edges = HashMap::new();

        for j in 0..g.N {
            for i in 0..g.N {
                let pos = [j,i];
                let thischar = g.get_pos(pos);
                if thischar == '#' {
                    continue;
                }
                let mut thisedges = HashMap::new();
                for dir in [[0i64,1],[0,-1],[1,0],[-1,0]].iter() {
                    if !p2 {
                        match thischar {
                            '>' => if dir != &[0,1] { continue; },
                            '<' => if dir != &[0,-1] { continue; },
                            'v' => if dir != &[1,0] { continue; },
                            '^' => if dir != &[-1,0] { continue; },
                            _ => (),
                        };
                    }
                    let _newpos = [pos[0] as i64 + dir[0], pos[1] as i64 + dir[1]];
                    if _newpos[0] < 0 || _newpos[0] >= g.N as i64 || _newpos[1] < 0 || _newpos[1] >= g.N as i64 {
                        continue;
                    }
                    let newpos = [_newpos[0] as usize, _newpos[1] as usize];
                    let newchar = g.get_pos(newpos);
                    if newchar == '#' {
                        continue;
                    }
                    let newid = newpos[0] * g.N + newpos[1];
                    thisedges.insert(newid, 1);
                }
                edges.insert(j * g.N + i, thisedges);
            }
        }
        Self {
            edges,
            p2,
            start: g.start[0] * g.N + g.start[1],
            end: g.end[0] * g.N + g.end[1],
            N: g.N,
        }
    }

    fn find_longest_path(&self, node: usize, mut visited: HashSet<usize>) -> Option<usize> {
        if node == self.end {
            return Some(0);
        }

        visited.insert(node);

        let mut maxlen: Option<usize> = None;
        if let Some(edges) = self.edges.get(&node) {
            for (newid,weight) in edges {
                if visited.contains(&newid) {
                    continue;
                }
                if let Some(pathlen) = self.find_longest_path(*newid, visited.clone()) {
                    maxlen = Some(match maxlen {
                        Some(x) => x.max(pathlen + weight),
                        None => pathlen + weight,
                    });
                }
            }
        }
        return maxlen;
    }

    fn print_edges(&self) {
        for (k,v) in self.edges.iter() {
            print!("[{},{}]: ", k / self.N, k % self.N);
            for (e,w) in v {
                print!("[{},{}]-{} ", e / self.N, e % self.N, w);
            }
            println!();
        }
    }

    fn contract(&mut self) {
        loop {
            if let Some((node,thisedges)) = self.edges.clone().iter().find(|(_,v)| v.len() == 2) {
                // println!("Contracting node {:?} with edges {:?}", node, thisedges);

                self.edges.remove(node);
                let leftid = thisedges.keys().next().unwrap();
                let rightid = thisedges.keys().last().unwrap();
                
                let left = self.edges.get_mut(&leftid).unwrap();
                left.remove(node);
                left.insert(*rightid, thisedges[leftid] + thisedges[rightid]);

                if thisedges.len() > 1 {
                    let right = self.edges.get_mut(&rightid).unwrap();
                    right.remove(node);
                    right.insert(*leftid, thisedges[leftid] + thisedges[rightid]);
                }
            }
            else {
                break;
            }
        }
    }
}

fn part2(txt: &str) -> usize {
    let mut grid = SparseGrid::new(txt, true);
    grid.contract();
    grid.find_longest_path(grid.start, HashSet::new()).unwrap()

    // let grid = Grid::new(txt, true);
    // grid.find_longest_path(grid.start, HashSet::new())
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
