#![allow(non_snake_case)]

use std::fs;

fn hash(txt: &str, start: usize) -> usize {
    // txt.chars()
    //     .map(|c| (c as usize)*17)
    //     .map(|c| {println!("{}", c); c} )
    //     .sum::<usize>() % 256usize
    let mut h = start;
    for c in txt.chars() {
        h += c as usize;
        h *= 17;
    }
    h % 256usize
}

fn part1(txt: &str) -> usize {
    txt.split(",")
        .map(|s| hash(&s, 0))
        // .map(|h| {println!("{}", h); h})
        .sum()
}

fn part2(txt: &str) -> usize {
    let mut boxes: Vec<Vec<(&str,usize)>> = vec![vec![];256];

    for _instr in txt.split(",") {
        let instr = _instr.split(['=','-']).collect::<Vec<&str>>();
        let l = instr[0];
        let f = instr[1].parse::<usize>();
        let h = hash(l, 0);
        let b = &mut boxes[h];
        if _instr.contains("=") {
            let insertat = b.iter().position(|&x| x.0 == l);
            if let Some(i) = insertat {
                b[i] = (l, f.unwrap());
            } else {
                b.push((l, f.unwrap()));
            }
        } else {
            b.iter().position(|&x| x.0 == l).map(|i| b.remove(i));
        }
    }

    boxes.iter().enumerate()
        .map(|(i,b)| b.iter().enumerate()
            .map(|(j,l)| (i+1)*(j+1)*l.1)
            .sum::<usize>())
        .sum::<usize>()
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
