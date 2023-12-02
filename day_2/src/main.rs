use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
struct Game {
    id: usize,
    draws: Vec<Draw>,
}

#[derive(Debug)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

fn main() {
    part_1();
    part_2();
}

impl Draw {
    fn parse(line: &str) -> Self {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for color in line.split(',') {
            if let Some(n) = color.strip_suffix("red") {
                red = n.trim().parse().unwrap();
            } else if let Some(n) = color.strip_suffix("green") {
                green = n.trim().parse().unwrap();
            } else if let Some(n) = color.strip_suffix("blue") {
                blue = n.trim().parse().unwrap();
            }
        }
        Draw { red, green, blue }
    }

    fn valid(&self, other: &Self) -> bool {
        return self.red <= other.red && self.green <= other.green && self.blue <= other.blue;
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl Game {
    fn parse(line: &str) -> Self {
        let (name, body) = line.split_once(':').unwrap();
        let id = name.strip_prefix("Game ").unwrap();

        let mut draws = Vec::new();
        for s in body.split(';') {
            draws.push(Draw::parse(s))
        }
        Game {
            id: id.parse().unwrap(),
            draws,
        }
    }

    fn compatible(&self, draw: &Draw) -> bool {
        self.draws.iter().all(|d| d.valid(draw))
    }

    fn min_set(&self) -> Draw {
        let red = self.draws.iter().map(|d| d.red).max().unwrap();
        let green = self.draws.iter().map(|d| d.green).max().unwrap();
        let blue = self.draws.iter().map(|d| d.blue).max().unwrap();
        Draw { red, green, blue }
    }
}

fn part_1() {
    let file = File::open("input.txt").unwrap();
    let mut sum = 0;

    let test_draw = Draw::parse("12 red, 13 green, 14 blue");
    for line in io::BufReader::new(file).lines().map(|x| x.unwrap()) {
        let game = Game::parse(&line);
        if game.compatible(&test_draw) {
            sum += game.id;
        }
    }
    println!("Total value is: {sum}");
}

fn part_2() {
    let file = File::open("input.txt").unwrap();
    let mut sum = 0;

    for line in io::BufReader::new(file).lines().map(|x| x.unwrap()) {
        let game = Game::parse(&line);
        let min_set = game.min_set();
        sum += min_set.power();
    }
    println!("Total value is: {sum}");
}
