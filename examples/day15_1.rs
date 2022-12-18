use std::collections::HashMap;
use glam::IVec2;
use nom::bytes::complete::tag;
use nom::character::complete::{char, one_of};
use nom::combinator::{map, recognize};
use nom::multi::{many0, many1};
use nom::sequence::terminated;

#[derive(Debug, PartialEq)]
enum Content {
    Sensor,
    Beacon,
    DefinitelyNoBeacon,
}

#[derive(Debug)]
struct Grid {
    cells: HashMap<IVec2, Content>,
    pairs: Vec<(IVec2, IVec2)>,
}

fn decimal(input: &str) -> nom::IResult<&str, i32> {
    map(
        recognize(many1(terminated(one_of("-0123456789"), many0(char('_'))))),
        |v| i32::from_str_radix(v, 10).expect("number"),
    )(input)
}

fn parse_line(input: &str) -> nom::IResult<&str, (IVec2, IVec2)> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, sx) = decimal(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, sy) = decimal(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, bx) = decimal(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, by) = decimal(input)?;
    Ok((input, (IVec2::new(sx, sy), IVec2::new(bx, by))))
}

fn manhattan(a: IVec2, b: IVec2) -> u32 {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

fn parse(input: &str, focus_y: Option<i32>) -> Grid {
    let pairs: Vec<(IVec2, IVec2)> = input.lines()
        .map(parse_line)
        .map(|result| result.ok().unwrap().1)
        .collect();

    let mut cells = HashMap::new();
    for (sensor, beacon) in &pairs {
        cells.insert(*sensor, Content::Sensor);
        let dist = manhattan(*sensor, *beacon) as i32;
        let start_x = sensor.x - dist;
        let mut start_y = sensor.y - dist;
        let end_x = sensor.x + dist;
        let mut end_y = sensor.y + dist;

        if let Some(focus_y) = focus_y {
            if (start_y < focus_y && end_y < focus_y) || (start_y > focus_y && end_y > focus_y) {
                continue;
            }

            start_y = focus_y;
            end_y = focus_y;
        }
        for y in start_y..=end_y {
            for x in start_x..=end_x {
                let pos = IVec2::new(x, y);
                if manhattan(*sensor, pos) as i32 <= dist {
                    if pos == *beacon {
                        cells.insert(pos, Content::Beacon);
                    } else {
                        if !cells.contains_key(&pos) {
                            cells.insert(pos, Content::DefinitelyNoBeacon);
                        }
                    }
                }
            }
        }
    }

    Grid {
        cells,
        pairs
    }
}

fn main() {
    let grid = parse(include_str!("day15.txt"), Some(2000000));
    let count = grid.cells.iter()
        .filter(|(cell, content)| {
            **content == Content::DefinitelyNoBeacon
        })
        .count();
    println!("count at row 2000000: {:?}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";
        let grid = parse(input, None);
        for y in 0..=22 {
            let mut str = "".to_string();
            for x in -2..=25 {
                let c = match grid.cells.get(&IVec2::new(x, y)) {
                  Some(Content::Sensor) => "S",
                    Some(Content::Beacon) => "B",
                    Some(Content::DefinitelyNoBeacon) => "#",
                    None => "."
                };
                str += c;
            }
            println!("{}", str);
        }

        let count = grid.cells.iter()
            .filter(|(cell, content)| {
                cell.y == 10 && **content == Content::DefinitelyNoBeacon
            })
            .count();
        assert_eq!(count, 26);
    }
}
