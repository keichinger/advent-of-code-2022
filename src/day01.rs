mod calories_data;
mod elf_calories_calculator;
mod elf_calories_stats;

use crate::day01::elf_calories_calculator::calculate_elf_calories_stats;
use num_format::{Locale, ToFormattedString};

pub fn run_problem() {
    println!("Advent of Code: Day 01");
    println!("======================");
    println!();

    let stats = calculate_elf_calories_stats();

    println!(
        "  - The elf carrying the highest number of calories is number {} with {} calories!",
        stats.highest_individual_calories_elf_index,
        stats
            .highest_individual_calories
            .to_formatted_string(&Locale::de)
    );
    println!(
        "  - The Top 3 total calories snack count is {} calories!",
        stats
            .top_three_total_calories
            .to_formatted_string(&Locale::de)
    );
}
