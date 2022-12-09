use std::collections::BTreeSet;
use glam::IVec2;

#[derive(Default)]
struct Rope {
    head: IVec2,
    knots: Vec<IVec2>,
    visited_tail_positions: BTreeSet<(i32, i32)>
}

impl Rope {
    pub fn new(num_knots: usize) -> Self {
        Self {
            head: IVec2::ZERO,
            knots: vec![IVec2::ZERO; num_knots],
            visited_tail_positions: BTreeSet::new()
        }
    }

    pub fn move_head(&mut self, offset: IVec2) {
        self.head += offset;

        let mut prev_knot = self.head;
        for knot in &mut self.knots {
            let diff = prev_knot - *knot;
            if diff.abs().max_element() > 1 {
                *knot += diff.clamp(IVec2::NEG_ONE, IVec2::ONE);
            }
            prev_knot = *knot;
        }

        let tail = self.knots.last().unwrap();
        self.visited_tail_positions.insert((tail.x, tail.y));
    }

    pub fn num_tail_positions(&self) -> usize {
        self.visited_tail_positions.len()
    }
}

fn read_moves(input: &str) -> Vec<IVec2> {
    input.lines()
        .map(|line| {
            let (dir, amount) = line.split_at(1);
            let amount = usize::from_str_radix(amount.trim(), 10).unwrap() as i32;

            match dir {
                "R" => IVec2::new(amount, 0),
                "L" => IVec2::new(-amount, 0),
                "U" => IVec2::new(0, -amount),
                "D" => IVec2::new(0, amount),
                _ => panic!("lol wat")
            }
        })
        .collect()
}

fn apply_moves(num_knots: usize, input: &str) -> Rope {
    let mut rope = Rope::new(num_knots);
    let moves = read_moves(input);
    for m in moves {
        let amount = m.abs().max_element();
        let m = m.clamp(IVec2::NEG_ONE, IVec2::ONE);

        for _ in 0..amount {
            rope.move_head(m);
        }
    }
    rope
}

fn main() {
    println!("num tail positions with 1 knot: {}", apply_moves(1, include_str!("day9.txt")).num_tail_positions());
    println!("num tail positions with 9 knots: {}", apply_moves(9, include_str!("day9.txt")).num_tail_positions());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
        let moves = read_moves(input);
        assert_eq!(moves[1], IVec2::new(0, -4));

        let rope = apply_moves(1, input);
        assert_eq!(rope.num_tail_positions(), 13);
    }
}
