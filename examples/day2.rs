#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    pub fn from(c: char) -> Shape {
        match c {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!()
        }
    }

    pub fn score(&self) -> u32 {
        match &self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    pub fn from(c: char) -> Outcome {
        match c {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!()
        }
    }

    pub fn score(&self) -> u32 {
        match &self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

fn calc_score(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            let theirs = Shape::from(line.chars().nth(0).unwrap());
            let ours = Shape::from(line.chars().nth(2).unwrap());

            let outcome = match (&theirs, &ours) {
                (Shape::Paper, Shape::Scissors) => Outcome::Win,
                (Shape::Rock, Shape::Paper) => Outcome::Win,
                (Shape::Scissors, Shape::Rock) => Outcome::Win,
                (_, _) if theirs == ours => Outcome::Draw,
                _ => Outcome::Lose
            };

            ours.score() + outcome.score()
        })
        .sum()
}

fn calc_score_part2(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            let theirs = Shape::from(line.chars().nth(0).unwrap());
            let outcome = Outcome::from(line.chars().nth(2).unwrap());

            let ours = match (&theirs, &outcome) {
                (Shape::Rock, Outcome::Win) => Shape::Paper,
                (Shape::Rock, Outcome::Lose) => Shape::Scissors,
                (Shape::Paper, Outcome::Win) => Shape::Scissors,
                (Shape::Paper, Outcome::Lose) => Shape::Rock,
                (Shape::Scissors, Outcome::Win) => Shape::Rock,
                (Shape::Scissors, Outcome::Lose) => Shape::Paper,
                (_, Outcome::Draw) => theirs,
            };

            ours.score() + outcome.score()
        })
        .sum()
}

pub fn main() {
    println!("score: {}", calc_score(include_str!("day2.txt")));
    println!("score part 2: {}", calc_score_part2(include_str!("day2.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let score = calc_score(r"A Y
B X
C Z");
        assert_eq!(score, 15);
    }
}
