use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut count = 0;
    for line in io::BufReader::new(file).lines().map(|x| x.unwrap()) {
        println!("{line}");
        count += 1;
    }
    println!("Total value is: {count}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn part_2() {
        panic!("Make this test fail");
    }
}