use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
};

#[derive(Hash, Debug, Eq, PartialEq)]
struct Position(usize, usize);
impl Num {
    fn adjacents(&self) -> Vec<Position> {
        let mut adjacents = Vec::new();
        let x_min = if self.pos.0 == 0 { 0 } else { self.pos.0 - 1 };
        let y_min = if self.pos.1 == 0 { 0 } else { self.pos.1 - 1 };
        let x_max = self.pos.0 + self.val.len();
        let y_max = self.pos.1 + 1;

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                adjacents.push(Position(x, y));
            }
        }
        adjacents
    }
}

#[derive(Debug)]
struct Num {
    val: String,
    pos: Position,
}

#[derive(Debug)]
struct Symbol {
    val: String,
    neighbors: Vec<u32>,
}

fn parse(lines: String) -> (Vec<Num>, HashMap<Position, Symbol>) {
    let mut nums = Vec::new();
    let mut syms = HashMap::new();
    for (y, line) in lines.split('\n').enumerate() {
        let mut num = "".to_string();
        let mut start = 0;
        for (x, c) in line.chars().enumerate() {
            if c.is_numeric() {
                // if this is the start of the number, keep the position
                if num == "" {
                    start = x;
                }
                num.push(c);
            } else {
                // if we were recording a number, store it and clear num
                if num != "" {
                    nums.push(Num {
                        val: num,
                        pos: Position(start, y),
                    });
                    num = "".to_string();
                }
            }
            if c != '.' && c.is_ascii_punctuation() {
                syms.insert(
                    Position(x, y),
                    Symbol {
                        val: c.to_string(),
                        neighbors: Vec::new(),
                    },
                );
            }
        }
        // if we were recording a number, store it and clear num
        if num != "" {
            nums.push(Num {
                val: num.parse().unwrap(),
                pos: Position(start, y),
            });
        }
    }
    return (nums, syms);
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut lines = String::new();
    io::BufReader::new(file)
        .read_to_string(&mut lines)
        .expect("Unable to read values");
    part_1(lines.clone());
    part_2(lines);
}

fn part_1(lines: String) {
    let mut sum = 0;
    let (nums, syms) = parse(lines);
    for n in nums {
        let adjacents = n.adjacents();
        for a in adjacents {
            if syms.contains_key(&a) {
                sum += n.val.parse::<u32>().unwrap();
                break;
            }
        }
    }
    println!("Sum: {sum}");
}

fn part_2(lines: String) {
    let mut sum = 0;
    let (nums, mut syms) = parse(lines);
    for n in nums {
        let adjacents = n.adjacents();
        for a in adjacents {
            if let Some(sym) = syms.get_mut(&a) {
                sym.neighbors.push(n.val.parse().unwrap())
            }
        }
    }

    for sym in syms.values() {
        if sym.val == "*" && sym.neighbors.len() == 2 {
            let ratio: u32 = sym.neighbors.iter().product();
            sum += ratio
        }
    }
    println!("Sum: {sum}");
}