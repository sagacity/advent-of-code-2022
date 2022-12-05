use crate::CrateMover::Cm9001;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{char, multispace0, one_of};
use nom::combinator::{map, recognize};
use nom::multi::{many0, many0_count, many1, separated_list1};
use nom::sequence::{delimited, preceded, terminated};
use nom::IResult;
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq)]
enum CrateMover {
    Cm9000,
    Cm9001,
}

#[derive(Default)]
struct Ship {
    stacks: Vec<VecDeque<char>>,
}

impl Ship {
    pub fn apply(&mut self, instruction: Instruction, crate_mover: CrateMover) {
        let mut removed = vec![];
        for _ in 0..instruction.count {
            removed.push(self.stacks[instruction.src - 1].pop_back().unwrap());
        }

        if crate_mover == Cm9001 {
            removed.reverse();
        }

        for r in removed {
            self.stacks[instruction.dst - 1].push_back(r);
        }
    }
}

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, crate_id) = delimited(char('['), take(1usize), char(']'))(input)?;
    Ok((input, Some(crate_id)))
}

fn parse_empty_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, _) = tag("   ")(input)?;
    Ok((input, None))
}

fn parse_line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    separated_list1(tag(" "), alt((parse_crate, parse_empty_crate)))(input)
}

fn parse_ship(input: &str) -> Ship {
    let (input, lines) = separated_list1(tag("\n"), parse_line)(input).unwrap();
    let (_, num_stacks) =
        many0_count(delimited(multispace0, decimal, multispace0))(input).unwrap();

    let mut stacks = vec![];
    for _ in 0..num_stacks {
        stacks.push(VecDeque::new());
    }

    for line in lines {
        for (idx, crate_id) in line.into_iter().enumerate() {
            if let Some(crate_id) = crate_id {
                let stack = stacks.get_mut(idx).unwrap();
                stack.push_front(crate_id.chars().next().unwrap());
            }
        }
    }

    Ship { stacks }
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    src: usize,
    dst: usize,
}

fn decimal(input: &str) -> IResult<&str, usize> {
    map(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |v| usize::from_str_radix(v, 10).expect("number"),
    )(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, count) = delimited(tag("move "), decimal, tag(" from "))(input)?;
    let (input, src) = decimal(input)?;
    let (input, dst) = preceded(tag(" to "), decimal)(input)?;
    Ok((input, Instruction { count, src, dst }))
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let (_, result) = separated_list1(tag("\n"), parse_instruction)(input).unwrap();
    result
}

fn parse(input: &str) -> (Ship, Vec<Instruction>) {
    let (ship, instructions) = input.split_once("\n\n").unwrap();

    (parse_ship(ship), parse_instructions(instructions))
}

fn calc_result(input: &str, crate_mover: CrateMover) -> String {
    let (mut ship, instructions) = parse(input);

    for i in instructions {
        ship.apply(i, crate_mover);
    }

    ship.stacks
        .into_iter()
        .map(|mut s| s.pop_back().unwrap())
        .collect()
}

pub fn main() {
    println!(
        "result: {}",
        calc_result(include_str!("day5.txt"), CrateMover::Cm9000)
    );
    println!(
        "result: {}",
        calc_result(include_str!("day5.txt"), CrateMover::Cm9001)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_input = r"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let (ship, instructions) = parse(test_input);
        assert_eq!(ship.stacks.get(1).unwrap(), &['M', 'C', 'D']);
        assert_eq!(instructions.len(), 4);

        let i = instructions.get(1).unwrap();
        assert_eq!(i.count, 3);
        assert_eq!(i.src, 1);
        assert_eq!(i.dst, 3);

        let result = calc_result(test_input, CrateMover::Cm9000);
        assert_eq!(result, "CMZ");

        let result = calc_result(test_input, CrateMover::Cm9001);
        assert_eq!(result, "MCD");
    }
}
