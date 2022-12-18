use std::collections::HashMap;
use glam::IVec2;
use itertools::Itertools;
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

fn parse(input: &str) -> Grid {
    let pairs: Vec<(IVec2, IVec2)> = input.lines()
        .map(parse_line)
        .map(|result| result.ok().unwrap().1)
        .collect();

    Grid {
        pairs
    }
}

fn main() {
    let input = include_str!("day15.txt");
    /*let input = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
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
";*/
    let max = IVec2::splat(4000000);
    //let max = IVec2::splat(20);
    let grid = parse(input);

    let mut options = HashMap::new();
    for (sensor, beacon) in &grid.pairs {
        let diff_1 = manhattan(*sensor, *beacon) + 1;
        let left = *sensor + IVec2::new(-(diff_1 as i32), 0);
        let bot = *sensor + IVec2::new(0, (diff_1 as i32));
        let right = *sensor + IVec2::new((diff_1 as i32), 0);
        let top = *sensor + IVec2::new(0, -(diff_1 as i32));

        let in_range_of_sensor = |pos: IVec2| {
            for (sensor, beacon) in &grid.pairs {
                let dist = manhattan(*sensor, *beacon);
                let dist_pos = manhattan(*sensor, pos);
                if dist_pos <= dist {
                    return true;
                }
            }
            false
        };

        let mut pos = left;
        while pos != bot {
            if !in_range_of_sensor(pos) && pos.x >= 0 && pos.y >= 0 && pos.x < max.x && pos.y < max.y { *options.entry(pos).or_insert(0) += 1; }
            pos.x += 1;
            pos.y += 1;
        }
        while pos != right {
            if !in_range_of_sensor(pos) && pos.x >= 0 && pos.y >= 0 && pos.x < max.x && pos.y < max.y { *options.entry(pos).or_insert(0) += 1; }
            pos.x += 1;
            pos.y -= 1;
        }
        while pos != top {
            if !in_range_of_sensor(pos) && pos.x >= 0 && pos.y >= 0 && pos.x < max.x && pos.y < max.y { *options.entry(pos).or_insert(0) += 1; }
            pos.x -= 1;
            pos.y -= 1;
        }
        while pos != left {
            if !in_range_of_sensor(pos) && pos.x >= 0 && pos.y >= 0 && pos.x < max.x && pos.y < max.y { *options.entry(pos).or_insert(0) += 1; }
            pos.x -= 1;
            pos.y += 1;
        }
        //*options.entry(pos).or_insert(0) += 1;
    }
    println!("{:#?}", options.values());
    let mut options = options.into_iter().map(|(pos, cnt)| (pos, cnt)).collect_vec();
    options.sort_by_key(|(pos, cnt)| *cnt);
    let options = options.into_iter().rev().take(1).map(|(pos, _)| pos).collect_vec();
    let pos = options.first().unwrap();
    println!("{:#?}", pos);
    println!("{:#?}", (pos.x as u64) * 4000000u64 + (pos.y as u64));
}
