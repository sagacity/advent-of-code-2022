use gcollections::ops::Subset;
use gcollections::ops::Overlap;
use interval::Interval;
use interval::ops::Range;
use itertools::Itertools;

fn calc_subsets(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            let split = line.split(',').collect_vec();
            let l = interval(split[0]);
            let r = interval(split[1]);
            if l.is_subset(&r) || r.is_subset(&l) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn calc_overlaps(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            let split = line.split(',').collect_vec();
            let l = interval(split[0]);
            let r = interval(split[1]);
            if l.overlap(&r) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn interval(input: &str) -> Interval<u32> {
    let (l, r) = input.split_once('-').unwrap();
    Interval::new(l.parse().unwrap(), r.parse().unwrap())
}

pub fn main() {
    println!("subsets: {}", calc_subsets(include_str!("day4.txt")));
    println!("overlaps: {}", calc_overlaps(include_str!("day4.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_subsets() {
        let num_subsets = calc_subsets(r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
        assert_eq!(num_subsets, 2);
    }

    #[test]
    pub fn test_overlaps() {
        let num_overlaps = calc_overlaps(r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
        assert_eq!(num_overlaps, 4);
    }
}
