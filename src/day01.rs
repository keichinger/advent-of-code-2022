mod calories_data;

use crate::day01::calories_data::CaloriesData;
use io::{BufReader, Lines};
use num_format::{Locale, ToFormattedString};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn run_problem() {
    println!("Advent of Code: Day 01");
    println!("======================");
    println!();

    let mut calories_entries = vec![];

    for (index, grouped_calories) in get_grouped_calories().iter().enumerate() {
        calories_entries.push(CaloriesData {
            elf_index: index + 1,
            calories: grouped_calories.to_vec(),
        })
    }

    println!(
        "  - Found calories data for {} elves.",
        calories_entries.len()
    );

    calories_entries.sort_by(|a, b| b.total_calories().cmp(&a.total_calories()));

    let highest_carrying_elf = calories_entries.first().unwrap();

    println!(
        "  - The elf carrying the highest number of calories is number {} with {} calories!",
        highest_carrying_elf.elf_index,
        highest_carrying_elf
            .total_calories()
            .to_formatted_string(&Locale::de)
    );
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn get_grouped_calories() -> Vec<Vec<i32>> {
    let mut data: Vec<Vec<i32>> = vec![];
    let mut current: Vec<i32> = vec![];

    if let Ok(lines) = read_lines("src/day01/input.txt") {
        for line in lines {
            if let Ok(calories_line) = line {
                if calories_line == "" {
                    data.push(current);

                    current = vec![];
                    continue;
                }

                current.push(calories_line.parse().unwrap());
            }
        }
    }

    if 0 < current.len() {
        data.push(current);
    }

    data
}
