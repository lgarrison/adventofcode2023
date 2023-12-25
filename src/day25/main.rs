#![allow(non_snake_case)]

use std::{fs, collections::{HashMap, HashSet}};

#[derive(Debug, Clone)]
struct Graph {
    edges: HashMap<String,HashSet<String>>,
}

impl Graph {
    fn new(txt: &str) -> Graph {
        let mut edges = HashMap::new();
        for line in txt.lines() {
            let mut thisedges = HashSet::new();
            line.split(' ').skip(1).for_each(|x| {
                thisedges.insert(x.to_string());
            });
            edges.insert(line.split(':').next().unwrap().to_string(), thisedges);
        }

        // make bidirectional
        let mut biedges = edges.clone();
        for (k, v) in edges.iter() {
            for vv in v.iter() {
                biedges.entry(vv.to_string()).or_insert(HashSet::new()).insert(k.to_string());
            }
        }
        
        Graph { edges: biedges }
    }

    fn group_sizes(&self) -> Vec<usize> {
        let mut sizes = vec![];
        let mut visited: HashSet<String> = HashSet::new();
        for node in self.edges.keys() {
            if !visited.contains(node) {
                sizes.push(
                    self.dfs(node.to_string(), &mut visited)
                );
            }
        }
        sizes
    }

    fn dfs(&self, node: String, visited: &mut HashSet<String>) -> usize {
        visited.insert(node.to_string());
        let mut size = 1;
        for n in self.edges.get(&node).unwrap().iter() {
            if !visited.contains(n.as_str()) {
                size += self.dfs(n.to_string(), visited);
            }
        }
        size
    }

    fn cost(&self, a: &String, B: &HashSet<String>) -> i64 {
        self.edges.get(a).unwrap()
            .iter()
            .filter(|&x| B.contains(x))
            .count() as i64
    }

    // fn kernighan_lin(&self) -> [HashSet<String>; 2] {
    //     let mut A: HashSet<String> = HashSet::new();
    //     let mut B: HashSet<String> = HashSet::new();
    //     let N = self.edges.len();

    //     self.edges.keys().enumerate().for_each(|(i,x)| {
    //         if i < N/2 {
    //             A.insert(x.to_string());
    //         } else {
    //             B.insert(x.to_string());
    //         }
    //     });

    //     while true {
    //         let Ia: Vec<i64> = A.iter().map(|x| self.cost(x, &A)).collect();
    //         let Ea: Vec<i64> = A.iter().map(|x| self.cost(x, &B)).collect();
    //         let Da: Vec<i64> = Ia.iter().zip(Ea.iter()).map(|(x,y)| x-y).collect();

    //         let Ib: Vec<i64> = B.iter().map(|x| self.cost(x, &B)).collect();
    //         let Eb: Vec<i64> = B.iter().map(|x| self.cost(x, &A)).collect();
    //         let Db: Vec<i64> = Ib.iter().zip(Eb.iter()).map(|(x,y)| x-y).collect();

    //         for i in 0..N/2 {
    //             // find a from A and b from B, such that g = D[a] + D[b] − 2×c(a, b) is maximal
    //         }
    //     }
        
    //     [A,B]
    // }

    fn dijkstra(&self, start: String, end: String) -> Vec<String> {
        let mut dist: HashMap<String, i64> = HashMap::new();
        let mut prev: HashMap<String, String> = HashMap::new();
        let mut Q: HashSet<String> = HashSet::new();

        for node in self.edges.keys() {
            dist.insert(node.to_string(), std::i64::MAX);
            prev.insert(node.to_string(), "".to_string());
            Q.insert(node.to_string());
        }
        dist.insert(start.to_string(), 0);

        while !Q.is_empty() {
            let mut u = "".to_string();
            let mut mindist = std::i64::MAX;
            for node in Q.iter() {
                if dist.get(node).unwrap() < &mindist {
                    mindist = *dist.get(node).unwrap();
                    u = node.to_string();
                }
            }
            Q.remove(&u);

            if u == end {
                break;
            }

            for v in self.edges.get(&u).unwrap().iter() {
                let alt = dist.get(&u).unwrap() + 1;
                if alt < *dist.get(v).unwrap() {
                    dist.insert(v.to_string(), alt);
                    prev.insert(v.to_string(), u.to_string());
                }
            }
        }

        let mut path = vec![];
        let mut u = end;
        while u != start {
            path.push(u.to_string());
            u = prev.get(&u).unwrap().to_string();
        }
        path.push(start.to_string());
        path.reverse();
        path
    }

    fn kargers(&self) -> usize {
        let mut counts: HashMap<[String;2],usize> = HashMap::new();

        let n = 15;
        for a in self.edges.keys().take(n) {
            for b in self.edges.keys().skip(n).take(n) {
                if a == b {
                    continue;
                }
                let path: Vec<String> = self.dijkstra(a.to_string(), b.to_string());
                path.windows(2).for_each(|slice| {
                    let [x, y]: [&String; 2] = [&slice[0], &slice[1]];
                    counts.entry([x.to_string(), y.to_string()]).and_modify(|e| *e += 1).or_insert(1);
                });
            }
        }
        // get the 3 nodes with the highest counts
        let mut sorted: Vec<(&usize, &[String;2])> = counts.iter().map(|(k,v)| (v,k)).collect();
        sorted.sort();
        sorted.reverse();
        println!("sorted: {:?}", sorted);

        let mut g = self.clone();
        for i in 0..3 {
            let [a,b] = sorted[i].1;
            g.edges.get_mut(a).unwrap().remove(b);
            g.edges.get_mut(b).unwrap().remove(a);
        }

        let sizes = g.group_sizes();
        println!("sizes: {:?}", sizes);
        // assert!(sizes.len() == 2);
        sizes.iter().product()
    }
}

fn part1(txt: &str) -> usize {
    let g = Graph::new(txt);
    println!("{:?}", g);
    println!("sizes: {:?}", g.group_sizes());
    g.kargers()
}

fn part2(txt: &str) -> i64 {
    0
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
