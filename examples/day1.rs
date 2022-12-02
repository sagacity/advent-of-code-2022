use std::collections::HashMap;
use itertools::Itertools;

pub fn main() {
    let lines = include_str!("day1.txt").lines();

    let mut calories: HashMap<usize, usize> = HashMap::new();
    let mut elf: usize = 1;
    for line in lines {
        if line.is_empty() {
            elf += 1;
        } else {
            let e = calories.entry(elf).or_insert(0);
            *e += usize::from_str_radix(line, 10)?;
        }
    }

    let calories = calories.into_iter()
        .sorted_by(|a, b| a.1.cmp(&b.1))
        .map(|e| e.1)
        .rev()
        .collect_vec();

    let max_calories_single_elf: usize = *calories.first().unwrap();
    println!("{:?}", max_calories_single_elf);
    let max_calories: usize = calories[0..3].into_iter().sum();
    println!("{:?}", max_calories);
}
