#![allow(non_snake_case)]

use std::{fs, collections::{HashMap, VecDeque}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Bcast,
    Flip,
    Conj,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module {
    outputs: Vec<String>,
    state: HashMap<String,bool>,
    kind: Kind,
    name: String,
    sent: [usize;2],
}

impl Module {
    fn new(txt: &str) -> Module {
        let parts: Vec<&str> = txt.split(" -> ").collect();
        let lhs = parts[0];
        let outputs = parts[1].split(", ").map(|s| s.to_string()).collect();
        let kind = match lhs.chars().nth(0).unwrap() {
            'b' => Kind::Bcast,
            '%' => Kind::Flip,
            '&' => Kind::Conj,
            _ => panic!("Unknown kind"),
        };
        let name = match kind {
            Kind::Bcast => "broadcaster".to_string(),
            Kind::Flip => lhs[1..].to_string(),
            Kind::Conj => lhs[1..].to_string(),
        };
        let state = match kind {
            Kind::Bcast => HashMap::new(),
            Kind::Flip => {
                let mut h = HashMap::new();
                h.insert(name.to_string(), false);
                h
            },
            Kind::Conj => {
                let h = HashMap::new();
                h
            },
        };
        Module {
            outputs,
            state,
            kind,
            name,
            sent: [0,0],
        }
    }
}

#[derive(Debug)]
struct Machine {
    modules: HashMap<String,Module>,
}

impl Machine {
    fn new(txt: &str) -> Machine {
        let mut modules = HashMap::new();
        for line in txt.lines() {
            let m = Module::new(line);
            modules.insert(m.name.clone(), m);
        }

        for m in modules.clone().values() {
            for o in &m.outputs {
                // println!("{} -> {}", m.name, o);
                if let Some(target) = modules.get_mut(o) {
                    if target.kind == Kind::Conj {
                        target.state.insert(m.name.clone(), false);
                    }
                }
            }
        }

        Machine { modules }
    }

    fn push_the_button(&mut self) {
        // button send a low pulse
        self.modules.get_mut("broadcaster").unwrap().sent[0] += 1;

        let mut queue = VecDeque::new();
        queue.push_back((false, "broadcaster".to_string(), "button".to_string()));
        while let Some((high,target, from)) = queue.pop_front() {
            if let Some(mut m) = self.modules.get_mut(&target){
                match m.kind {
                    Kind::Bcast => {
                        let new = high;
                        for o in &m.outputs {
                            m.sent[new as usize] += 1;
                            queue.push_back((new, o.to_string(), m.name.clone()));
                        }
                    },
                    Kind::Flip => {
                        if high {
                            continue;
                        }
                        m.state.insert(m.name.clone(), !m.state[&m.name]);
                        let new = m.state[&m.name];
                        for o in &m.outputs {
                            m.sent[new as usize] += 1;
                            queue.push_back((new, o.to_string(), m.name.clone()));
                        }
                    },
                    Kind::Conj => {
                        m.state.insert(from, high);
                        let allhigh = m.state.values().all(|&x| x);
                        let new = !allhigh;
                        for o in &m.outputs {
                            m.sent[new as usize] += 1;
                            queue.push_back((new, o.to_string(), m.name.clone()));
                        }
                    },
                }
            }
        }
    }

    fn score(&self) -> usize {
        (0..=1).map(|i| self.modules.iter().map(|(_,m)| m.sent[i]).sum::<usize>())
            .product()
    }
}

fn part1(txt: &str) -> usize {
    let mut m = Machine::new(txt);
    for _ in 0..1000 {
        m.push_the_button();
    }
    // for mm in m.modules.values() {
    //     println!("{: >12}: {:?}", mm.name, mm);
    // }
    m.score()
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a%b)
    }
}

fn part2(txt: &str) -> i64 {
    let mut m = Machine::new(txt);
    let mut count = 0;
    let mut cycles = [0;4];
    for _ in 0..100_000 {
        m.push_the_button();
        count += 1;
        if cycles[0] == 0 && m.modules["cl"].sent[1] == 1 {
            cycles[0] = count;
        }
        if cycles[1] == 0 && m.modules["rp"].sent[1] == 1 {
            cycles[1] = count;
        }
        if cycles[2] == 0 && m.modules["lb"].sent[1] == 1 {
            cycles[2] = count;
        }
        if cycles[3] == 0 && m.modules["nj"].sent[1] == 1 {
            cycles[3] = count;
        }
    }
    println!("cycles: {:?}", cycles);
    lcm(lcm(cycles[0], cycles[1]), lcm(cycles[2], cycles[3]))
}

fn main() {
    let dayX = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + dayX + "/input.txt";
    // let path = String::from(root) + "/src/" + dayX + "/test1.txt";
    // let path = String::from(root) + "/src/" + dayX + "/test2.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", dayX);
    println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}
