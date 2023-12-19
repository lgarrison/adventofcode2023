#![allow(non_snake_case)]

use std::{fs, collections::HashMap};

#[derive(Debug)]
struct Workflow {
    rules: Vec<(usize,bool,usize,String)>,
    name: String,
}

struct Workflows {
    data: HashMap<String,Workflow>,
}

#[derive(Debug)]
struct Part([usize;4]);

#[derive(Debug, Clone, Copy)]
struct PartRange([(usize,usize);4]);

impl Workflow {
    fn new(line: &str) -> Workflow {
        let mut chunks = line.split(['{','}']);
        let name = chunks.next().unwrap().to_string();
        let maps = chunks.next().unwrap().split(',').collect::<Vec<&str>>();
        let mut rules: Vec<(usize,bool,usize,String)> = Vec::new();

        for m in &maps[..maps.len()-1] {
            let rule: Vec<&str> = m.split(':').collect();
            let field = match rule[0].chars().nth(0).unwrap() {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => panic!("Unknown field"),
            };
            let inequality = rule[0].chars().nth(1).unwrap();
            let val = rule[0].chars().skip(2).collect::<String>().parse::<usize>().unwrap();
            rules.push(
                match inequality {
                    '<' => (field, true, val, rule[1].to_string()),
                    '>' => (field, false, val, rule[1].to_string()),
                    _ => panic!("Unknown inequality"),
                }
            )
        }

        match maps[maps.len()-1] {
            "A" => rules.push((0,false,0,"A".to_string())),
            "R" => rules.push((0,false,0,"R".to_string())),
            s => rules.push((0,false,0,s.to_string())),
        }

        Workflow { rules, name }
    }
    
    fn apply(&self, part: &Part) -> String {
        let mut result = String::new();
        for rule in &self.rules {
            let val = part.0[rule.0];
            if (rule.1 && val < rule.2) || (!rule.1 && val > rule.2) {
                result = rule.3.clone();
                break;
            }
        }
        result
    }
}

impl Workflows {
    fn count_combinations(&self, name: &str, part: &PartRange) -> usize {
        // println!("{} {:?}", name, part);
        let w = self.data.get(name).unwrap();

        let mut rempart = part.clone();
        let mut sum = 0;
        for (field, lr,val,target) in &w.rules {
            let mut newpart: PartRange = rempart.clone();
            if *lr {
                let intersect = rempart.0[*field].1.min(*val);
                if intersect >= rempart.0[*field].0 {
                    newpart.0[*field] = (rempart.0[*field].0, intersect);
                    rempart.0[*field] = (intersect-1, rempart.0[*field].1);
                    sum += 
                        match target.as_str() {
                            "A" => newpart.0.iter().map(|(a,b)| (b-a)-1).product::<usize>(),
                            "R" => 0,
                            _ => self.count_combinations(target, &newpart),
                        };
                }
            } else {
                let intersect = rempart.0[*field].0.max(*val);
                if intersect <= rempart.0[*field].1 {
                    newpart.0[*field] = (intersect, rempart.0[*field].1);
                    rempart.0[*field] = (rempart.0[*field].0, intersect+1);
                    sum += 
                        match target.as_str() {
                            "A" => newpart.0.iter().map(|(a,b)| (b-a)-1).product::<usize>(),
                            "R" => 0,
                            _ => self.count_combinations(target, &newpart),
                        };
                }
            }
        }
        sum
    }
}

impl Part {
    fn new(line: &str) -> Part {
        let parts: Vec<&str> = line.split(['=',',','{','}']).collect();
        Part([
            parts[2].parse::<usize>().unwrap(),
            parts[4].parse::<usize>().unwrap(),
            parts[6].parse::<usize>().unwrap(),
            parts[8].parse::<usize>().unwrap(),
        ]
        )
    }

    fn sum(&self) -> usize {
        self.0.iter().sum()
    }
}

fn part1(txt: &str) -> i64 {
    let mut chunks = txt.split("\n\n");
    let mut workflows: HashMap<String,Workflow> = HashMap::new();
    
    for line in chunks.next().unwrap().lines(){
        let w = Workflow::new(line);
        workflows.insert(w.name.clone(), w);
    }

    let parts: Vec<Part> = chunks.next().unwrap()
        .lines()
        .map(|line| Part::new(line)).collect();

    let accepted: Vec<&Part> = parts.iter().filter(|p| {
        let mut name = "in".to_string();
        loop {
            name = workflows.get(&name).unwrap().apply(&p);
            if name == "R" {
                break false;
            }
            if name == "A" {
                break true;
            }
        }
    }).collect();

    accepted.iter().map(|p| p.sum()).sum::<usize>() as i64
}

fn part2(txt: &str) -> usize {
    let mut chunks = txt.split("\n\n");
    let mut workflows: HashMap<String,Workflow> = HashMap::new();
    
    for line in chunks.next().unwrap().lines(){
        let w = Workflow::new(line);
        workflows.insert(w.name.clone(), w);
    }

    let W = Workflows { data: workflows };
    W.count_combinations("in", &PartRange([(0,4001);4]))
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
