#![allow(non_snake_case)]

use std::{fs, cmp::Ordering};

// #[derive(Debug, Clone, Copy)]
// enum Card {
//     NUM(i64),
//     T,
//     J,
//     K,
//     Q,
//     A,
// }

type Card = i64;

fn card_from_char(c: char, p2: bool) -> Card {
    match c {
        'T' => 10,
        'J' => if p2 { 1 } else { 11 },
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => c.to_digit(10).unwrap() as i64,
    }
}

type HandType = i64;

#[derive(Debug, Clone, Copy, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: i64,
    // occurences: [i64; 15],
    hand_type: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type.cmp(&other.hand_type)
            .then_with(|| {
                for i in 0..5 {
                    if self.cards[i] != other.cards[i] {
                        return self.cards[i].cmp(&other.cards[i]);
                    }
                }
                Ordering::Equal
            })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Hand {
    fn from_str(s: &str, p2: bool) -> Hand {
        let mut cards = [0; 5];
        let mut occurences = [0; 15];
        let mut i = 0;
        let mut ss = s.split_whitespace();
        for c in ss.next().unwrap().chars() {
            cards[i] = card_from_char(c, p2);
            occurences[cards[i] as usize] += 1;
            i += 1;
        }

        let hand_type = {
            let mut sorted = occurences.clone();
            sorted.sort();
            sorted.reverse();

            let NJ = if p2 { occurences[1] } else { 0 };
            if sorted[0] == 5 {
                7  // five of a kind
            } else if sorted[0] == 4 {
                if NJ == 1 || NJ == 4 { 7 } else { 6 }   // four of a kind
            } else if sorted[0] == 3 && sorted[1] == 2 {
                if NJ == 3 || NJ == 2 { 7 } else { 5 } // full house
            } else if sorted[0] == 3 {
                if NJ == 1 || NJ == 3 { 6 } else { 4 } // three of a kind
            } else if sorted[0] == 2 && sorted[1] == 2 {
                if NJ == 1 { 5 } else if NJ == 2 { 6 } else { 3 } // two pairs
            } else if sorted[0] == 2 {
                if NJ == 2 || NJ == 1 { 4 } else { 2 } // one pair
            } else {
                if NJ == 1 { 2 } else { 1 } // high card
            }
        };

        Hand {
            cards,
            bid: ss.next().unwrap().parse::<i64>().unwrap(),
            hand_type,
        }
    }


}

fn part1(txt: &str) -> i64 {
    let mut hands = txt
        .lines()
        .map(|s| Hand::from_str(s, false))
        .collect::<Vec<Hand>>();
    // println!("{:?}", hands);
    hands.sort();
    // println!("{:?}", hands);

    hands.iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i as i64 + 1))
        .sum()
}

fn part2(txt: &str) -> i64 {
    let mut hands = txt
        .lines()
        .map(|s| Hand::from_str(s, true))
        .collect::<Vec<Hand>>();
    // println!("{:?}", hands);
    for h in hands.iter() {
        println!("{:?}", h);
    }
    hands.sort();
    println!("{:?}", hands);

    hands.iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i as i64 + 1))
        .sum()
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
