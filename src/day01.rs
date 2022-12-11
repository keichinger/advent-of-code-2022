mod calories_data;
mod elf_calories_calculator;

use crate::day01::elf_calories_calculator::calculate_elf_calories;
use num_format::{Locale, ToFormattedString};

pub fn run_problem() {
    println!("Advent of Code: Day 01");
    println!("======================");
    println!();

    let calories_entries = calculate_elf_calories();

    println!(
        "  - Found calories data for {} elves.",
        calories_entries.len()
    );

    let highest_carrying_elf = calories_entries.first().unwrap();

    println!(
        "  - The elf carrying the highest number of calories is number {} with {} calories!",
        highest_carrying_elf.elf_index,
        highest_carrying_elf
            .total_calories()
            .to_formatted_string(&Locale::de)
    );
}
