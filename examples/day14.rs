use std::collections::{HashMap, VecDeque};
use glam::IVec2;

#[derive(Clone, Debug)]
enum Content {
    Rock,
    Sand,
}

#[derive(Clone, Debug)]
struct Grid {
    cells: HashMap<IVec2, Content>,
    max_y: i32,
}

impl Grid {
    fn drop_sand(&mut self, use_max_y: bool) -> Option<IVec2> {
        let mut sand = IVec2::new(500, 0);
        while sand.y < 1000 {
            let new_pos = sand + IVec2::new(0, 1);
            if !self.is_blocked(new_pos, use_max_y) {
                sand = new_pos;
                continue;
            }

            let new_pos = sand + IVec2::new(-1, 1);
            if !self.is_blocked(new_pos, use_max_y) {
                sand = new_pos;
                continue;
            }

            let new_pos = sand + IVec2::new(1, 1);
            if !self.is_blocked(new_pos, use_max_y) {
                sand = new_pos;
                continue;
            }

            self.cells.insert(sand, Content::Sand);
            return Some(sand);
        }

        None
    }

    fn is_blocked(&self, pos: IVec2, use_max_y: bool) -> bool {
        if self.cells.contains_key(&pos) {
            return true;
        }

        if !use_max_y {
            return false;
        } else {
            pos.y >= self.max_y
        }
    }
}

fn parse(input: &str) -> Grid {
    let cells: HashMap<IVec2, Content> = input.lines()
        .map(|line| {
            let mut pts = line.split(" -> ").map(|str| {
                let (x, y) = str.split_once(",").unwrap();
                IVec2::new(i32::from_str_radix(x, 10).unwrap(), i32::from_str_radix(y, 10).unwrap())
            }).collect::<VecDeque<_>>();

            let mut result = vec![];
            let mut prev = pts.pop_front().unwrap();
            while let Some(pt) = pts.pop_front() {
                let mut cur = prev;
                let offset = (pt - cur).clamp(IVec2::NEG_ONE, IVec2::ONE);
                result.push((cur, Content::Rock));
                while cur != pt {
                    result.push((cur, Content::Rock));
                    cur += offset;
                }
                result.push((cur, Content::Rock));
                prev = pt;
            }
            result
        })
        .flatten()
        .collect();

    let max_y = cells.keys().map(|vec| vec.y).max().unwrap() + 2;

    Grid {
        cells,
        max_y
    }
}

fn main() {
    let mut grid = parse(include_str!("day14.txt"));
    let mut num_grains = 0;
    while grid.drop_sand(false).is_some() {
        num_grains += 1;
    }
    println!("{}", num_grains);

    let mut grid = parse(include_str!("day14.txt"));
    let mut num_grains = 0;
    while grid.drop_sand(true) != Some(IVec2::new(500, 0)) {
        num_grains += 1;
    }
    println!("{}", num_grains + 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut grid = parse(r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9");
        for y in 0..10 {
            let mut s = "".to_string();
            for x in 494..=503 {
                let cell = grid.cells.get(&IVec2::new(x, y));
                s += match cell {
                    Some(Content::Rock) => "#",
                    Some(Content::Sand) => "o",
                    None => ".",
                };
            }
            println!("{}", s);
        }

        let mut grid2 = grid.clone();
        /*let mut num_grains = 0;
        while grid.drop_sand(false).is_some() {
            num_grains += 1;
        }
        println!("{}", num_grains);*/

        let mut num_grains = 0;
        while grid2.drop_sand(true) != Some(IVec2::new(500, 0)) {
            num_grains += 1;
        }
        println!("{}", num_grains + 1);
    }
}
