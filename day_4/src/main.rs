use std::{
    collections::HashSet, fs,
};

fn main() {
    part_1();
    part_2()
}

fn part_1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut count = 0;
    for line in input.split_terminator('\n') {
        let (winning, have) = parse_card(line);
        count += score(winning, have);
    }
    println!("Total value is: {count}");
}

fn part_2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut cards: Vec<(&str, usize)> = input.split_terminator('\n').map(|l| (l, 1)).collect();
    let num_cards = cards.len();
    for i in 0..num_cards {
        let card = cards[i].clone();
        let (winning, have) = parse_card(card.0);
        let m = matches(winning, have);
        for _ in 0..card.1 {
            let end = usize::min(i + m + 1, num_cards);
            for k in i + 1..end as usize {
                cards[k].1 += 1;
            }
        }
    }
    println!(
        "Total value is: {}",
        cards.iter().map(|card| card.1 as u32).sum::<u32>()
    );
}

fn parse_nums(s: &str) -> Vec<u32> {
    s.split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn parse_card(line: &str) -> (HashSet<u32>, Vec<u32>) {
    let (_id, body) = line.split_once(':').unwrap();
    let (left, right) = body.split_once('|').unwrap();
    let winning = parse_nums(left);
    let have = parse_nums(right);
    (HashSet::from_iter(winning), have)
}

fn score(winning: HashSet<u32>, have: Vec<u32>) -> usize {
    let matches = matches(winning, have);
    if matches == 0 {
        return 0;
    }
    usize::pow(2, matches as u32 - 1)
}

fn matches(winning: HashSet<u32>, have: Vec<u32>) -> usize {
    have.iter().filter(|n| winning.contains(n)).count()
}