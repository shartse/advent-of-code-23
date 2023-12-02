use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let tuples = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let nums: HashMap<_, _> = tuples.into_iter().collect();

    let file = File::open("input.txt").unwrap();
    let mut count = 0;
    for line in io::BufReader::new(file).lines().map(|x| x.unwrap()) {
        let digit = calibration_digit(&line, &nums);
        println!("{line}: {digit}");

        count += digit;
    }
    println!("Total value is: {count}");
}

// the calibration value can be found by combining the
// first digit and the last digit (in that order) to form a single two-digit number.
fn calibration_digit(line: &str, nums: &HashMap<&str, u32>) -> u32 {
    let mut indices: HashMap<usize, u32> = HashMap::new();
    for num in nums.keys().filter(|&key| line.contains(key)) {
        for (i, _) in line.match_indices(num) {
            indices.insert(i, nums[num]);
        }
    }

    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() {
            indices.insert(i, format!("{c}").parse().unwrap());
        }
    }

    let mut values: Vec<(usize, u32)> = indices.into_iter().collect();
    values.sort_by(|a, b| a.0.cmp(&b.0));
    let first = values.first().expect("Should be a first").1;
    let second = values.last().expect("Should be a last").1;
    let digit = format!("{first}{second}");
    digit.parse().unwrap()
}