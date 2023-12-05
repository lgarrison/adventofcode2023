#![allow(non_snake_case)]

use std::{fs, ops::Range, cmp, vec};

#[derive(Debug, Clone)]
struct RangePair {
    dest: Range<i64>,
    source: Range<i64>,
}

impl RangePair {
    fn new(dest_start: i64, source_start: i64, count: i64) -> Self {
        Self {
            dest: dest_start..dest_start+count,
            source: source_start..source_start+count,
        }
    }

    fn intersect(&self, other: &Range<i64>) -> Vec<Option<Range<i64>>> {
        let mut ranges = vec![];
        ranges.push(
            if other.start < self.source.start {
                // unmapped region before self.source
                Some(other.start..cmp::min(other.end, self.source.start))
            } else {
                None
            }
        );
        ranges.push(
            if other.end > self.source.end {
                // unmapped region after self.source
                Some(cmp::max(other.start, self.source.end)..other.end)
            } else {
                None
            }
        );

        if other.end > self.source.start &&
            self.source.end > other.start {
            // mapped region
            let mut ret = self.dest.clone();
            if self.source.start < other.start {
                ret.start += other.start - self.source.start;
            }
            if self.source.end > other.end {
                ret.end -= self.source.end - other.end;
            }

            ranges.push(Some(ret));
        }
        else {
            ranges.push(None);
        }
        ranges
    }

}

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Vec<RangePair>>,
}

impl Almanac {
    fn new(txt: &str) -> Self {
        let mut lines = txt.lines();
        let seeds = lines.next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

        let sections = txt.split("\n\n").skip(1);
        let maps = sections.map(
            |sec| sec.split("\n")
            .skip(1)
            .filter(|line| line.len() > 0)
            .map(
                |line| {
                    let nums = line.split_whitespace()
                    .map(
                        |x| x.parse::<i64>().unwrap()
                    ).collect::<Vec<i64>>();
                    RangePair::new(nums[0], nums[1], nums[2])
                }
            ).collect()
        ).collect();

        Self {
            seeds,
            maps
        }
    }

    fn walk(&self, m: usize, range: Range<i64>) -> Vec<Range<i64>> {
        if m == self.maps.len() {
            return vec![range];
        }
        let mut ret: Vec<Range<i64>> = vec![];
        let mut unmatched = vec![range.clone()];
        let mut next_unmatched = vec![];
        for pair in &self.maps[m] {
            for r in unmatched.iter() {
                let ranges = pair.intersect(&r);
                if ranges.len() > 0 {
                    for unmatched_r in ranges[0..=1].iter() {
                        if let Some(ur) = unmatched_r {
                            next_unmatched.push(ur.clone());
                        }
                    }
                    if let Some(mr) = &ranges[2] {
                        ret.push(mr.clone());
                    }
                }
            }
            unmatched = next_unmatched;
            next_unmatched = vec![];
        }

        ret.extend(unmatched);

        ret.dedup();  // supposed to happen...?
        ret.iter()
            .flat_map(|r| self.walk(m+1, r.clone()))
            .collect()
    }

    fn seed_ranges(&self) -> Vec<Range<i64>> {
        self.seeds.iter()
        // .take(1)  // debug
        .flat_map(|&s|
            self.walk(0, s..s+1)
        )
        .collect()
    }

    fn seed_ranges_p2(&self) -> Vec<Range<i64>> {
        self.seeds
        .chunks(2)
        // .take(1)  // debug
        .flat_map(|s: &[i64]|
            self.walk(0, s[0]..s[0]+s[1])
        )
        .collect()
    }
}

fn part1(txt: &str) -> i64 {
    let almanac = Almanac::new(txt);
    let seed_ranges = almanac.seed_ranges();
    seed_ranges.iter().map(|r| r.start).min().unwrap()
}

fn part2(txt: &str) -> i64 {
    let almanac: Almanac = Almanac::new(txt);
    let seed_ranges = almanac.seed_ranges_p2();
    seed_ranges.iter().map(|r| r.start).min().unwrap()
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
