#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CaloriesData {
    pub elf_index: usize,
    pub calories: Vec<i32>,
}

impl CaloriesData {
    #[allow(dead_code)]
    pub fn total_calories(&self) -> i32 {
        self.calories.iter().sum()
    }
}
