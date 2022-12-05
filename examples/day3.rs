use itertools::Itertools;
use std::collections::BTreeSet;

fn calc_score(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_at(line.len() / 2);

            let l = l.chars().collect::<BTreeSet<_>>();
            let r = r.chars().collect::<BTreeSet<_>>();

            l.intersection(&r)
                .map(|c| match c {
                    'a'..='z' => (1 + (*c as u8 - 'a' as u8)) as u32,
                    'A'..='Z' => (27 + (*c as u8 - 'A' as u8)) as u32,
                    _ => panic!(),
                })
                .sum::<u32>()
        })
        .sum()
}

fn calc_score2(input: &str) -> u32 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            let a = chunk.next().unwrap().chars().collect::<BTreeSet<_>>();
            let b = chunk.next().unwrap().chars().collect::<BTreeSet<_>>();
            let c = chunk.next().unwrap().chars().collect::<BTreeSet<_>>();

            let i = a.intersection(&b).cloned().collect::<BTreeSet<_>>();

            i.intersection(&c)
                .map(|c| match c {
                    'a'..='z' => (1 + (*c as u8 - 'a' as u8)) as u32,
                    'A'..='Z' => (27 + (*c as u8 - 'A' as u8)) as u32,
                    _ => panic!(),
                })
                .sum::<u32>()
        })
        .sum()
}

pub fn main() {
    println!("score: {}", calc_score(include_str!("day3.txt")));
    println!("score: {}", calc_score2(include_str!("day3.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let score = calc_score(
            r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(score, 157);
    }

    #[test]
    pub fn test_part2() {
        let score = calc_score2(
            r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(score, 70);
    }
}
