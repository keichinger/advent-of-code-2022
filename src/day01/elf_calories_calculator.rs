use crate::day01::calories_data::CaloriesData;
use crate::day01::elf_calories_stats::ElfCaloriesStats;
use io::{BufReader, Lines};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn calculate_elf_calories_stats() -> ElfCaloriesStats {
    let mut calories_entries = vec![];

    for (index, grouped_calories) in get_grouped_calories().iter().enumerate() {
        calories_entries.push(CaloriesData {
            elf_index: index + 1,
            calories: grouped_calories.to_vec(),
        })
    }

    calories_entries.sort_by(|a, b| b.total_calories().cmp(&a.total_calories()));

    let highest_individual = calories_entries.first().unwrap();
    let highest_individual_calories_elf_index = highest_individual.elf_index;
    let highest_individual_calories = highest_individual.total_calories();

    let top_three_total_calories =
        calories_entries
            .into_iter()
            .take(3)
            .fold(0, |mut sum, value| {
                sum += value.total_calories();
                sum
            });

    ElfCaloriesStats {
        highest_individual_calories_elf_index,
        highest_individual_calories,
        top_three_total_calories,
    }
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
