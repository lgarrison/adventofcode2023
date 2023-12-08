#![allow(non_snake_case)]

use std::{fs, collections::{HashMap, HashSet}, str::Lines};

struct Node {
    left: String,
    right: String,
}

struct Network {
    nodes: HashMap<String,Node>,
}

impl Network {
    fn from_txt(lines: Lines) -> Network {
        let mut nodes = HashMap::new();

        for line in lines {
            let mut tokens = line.split([' ', '=', '(' , ')', ','])
                .filter(|&s| s.len() > 0);
            nodes.insert(
                tokens.next().unwrap().to_owned(),
                Node {
                    left: tokens.next().unwrap().to_owned(),
                    right: tokens.next().unwrap().to_owned(),
                });
        }

        Network{nodes}
    }

    fn get(&self, name: &str) -> &Node {
        self.nodes.get(name).unwrap()
    }

    fn find_len(&self, steps: &Vec<char>, target: &str) -> i64 {
        let mut cur = "AAA";
        let mut i = 0;
        for s in steps.iter().cycle() {
            if cur == target {
                return i;
            }
            match s {
                'L' => cur = &self.get(cur).left,
                'R' => cur = &self.get(cur).right,
                _ => panic!("Unknown step {}", s),
            }
            i += 1;
        }
        i
    }

    fn nodes_ending_with(&self, X: &str) -> Vec<&str> {
        self.nodes.keys()
            .filter(|&k| k.ends_with(X))
            .map(|s| s.as_str())
            .collect()
    }

    fn find_zs(&self, from: &str, steps: &Vec<char>) -> Vec<i64> {
        let mut cur = from;
        let mut i: i64 = 0;
        let mut seen_states: HashSet<(&str,i64)> = HashSet::new();
        let mut zdists: Vec<i64> = Vec::new();
        for s in steps.iter().cycle() {
            let state = (cur, i % steps.len() as i64);
            if seen_states.contains(&state) {
                break;
            }
            seen_states.insert(state);
            if cur.ends_with("Z") {
                zdists.push(i);
            }
            match s {
                'L' => cur = &self.get(cur).left,
                'R' => cur = &self.get(cur).right,
                _ => panic!("Unknown step {}", s),
            }
            i += 1;
        }
        zdists
    }
}

fn part1(txt: &str) -> i64 {
    let mut lines = txt.lines();
    let steps = lines.next().unwrap().chars().collect();
    lines.next();
    let network = Network::from_txt(lines);
    network.find_len(&steps, "ZZZ")
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}

fn lcm(nums: Vec<i64>) -> i64 {
    let mut cur = nums[0];
    for n in nums.iter().skip(1) {
        cur = cur * n / gcd(cur, *n);
    }
    cur
}

fn part2(txt: &str) -> i64 {
    // For each A node, find the number of steps to all Z nodes
    // Then find the smallest shared value in those sets
    let mut lines = txt.lines();
    let steps: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();
    let network = Network::from_txt(lines);

    // let mut lens: Vec<Vec<i64>> = Vec::new();
    let mut lens: Vec<Vec<i64>> = network.nodes_ending_with("A").iter()
        .map(|&s| network.find_zs(s, &steps))
        .collect();
    
    // LCM of the cycle lens
    let f = lcm(
        lens.iter()
            .map(|v| *v.last().unwrap() as i64)
            .collect()
        );
    
    for l in lens.iter_mut() {
        let cyclen = *l.last().unwrap();
        for v in l.iter_mut() {
            *v *= f / cyclen;
        }
    }

    let sets = lens.iter()
        .map(|v| v.iter().cloned().collect::<HashSet<i64>>())
        .collect::<Vec<HashSet<i64>>>();

    // smallest common value in sets
    let mut state = sets.get(0).unwrap().clone();
    for v in sets.iter().skip(1) {
        state = state.intersection(v).cloned().collect::<HashSet<i64>>();
    }

    *state.iter().min().unwrap()
}

fn main() {
    let dayX = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + dayX + "/input.txt";
    // let path = String::from(root) + "/src/" + dayX + "/test1.txt";
    // let path = String::from(root) + "/src/" + dayX + "/test2.txt";
    // let path = String::from(root) + "/src/" + dayX + "/test3.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", dayX);
    // println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}
