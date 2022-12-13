use std::cmp::Ordering;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, one_of};
use nom::combinator::{map, recognize};
use nom::multi::{many0, many1, separated_list0};
use nom::sequence::{delimited, terminated};

#[derive(Clone, Debug, PartialEq)]
enum Item {
    Value(usize),
    Array(Vec<Item>)
}
fn item_array(input: &str) -> nom::IResult<&str, Item> {
    let (input, item) = delimited(tag("["), separated_list0(tag(","), alt((item_array, item_value))), tag("]"))(input)?;
    Ok((input, Item::Array(item)))
}

fn item_value(input: &str) -> nom::IResult<&str, Item> {
    let (input, item) = decimal(input)?;
    Ok((input, Item::Value(item)))
}

fn decimal(input: &str) -> nom::IResult<&str, usize> {
    map(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |v| usize::from_str_radix(v, 10).expect("number"),
    )(input)
}

fn parse(input: &str) -> Vec<(Item, Item)> {
    input.split("\n\n")
        .map(|input| {
            let (l, r) = input.split_once("\n").unwrap();
            let (_, l) = item_array(l).unwrap();
            let (_, r) = item_array(r).unwrap();
            (l, r)
        })
        .collect()
}

impl PartialOrd for Item {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        let lhs = self;
        match (lhs, rhs) {
            (Item::Value(l), Item::Value(r)) => {
                if *l < * r {
                    return Some(Ordering::Less);
                }
                if *l > *r {
                    return Some(Ordering::Greater);
                }

                Some(Ordering::Equal)
            },
            (Item::Array(l), Item::Array(r)) => {
                for i in 0..l.len() {
                    if i >= r.len() {
                        return Some(Ordering::Greater);
                    }

                    let order = l[i].partial_cmp(&r[i]).unwrap();
                    if order != Ordering::Equal {
                        return Some(order);
                    }
                }

                if l.len() < r.len() {
                    Some(Ordering::Less)
                } else {
                    assert_eq!(l.len(), r.len());
                    Some(Ordering::Equal)
                }
            },
            (Item::Value(l), Item::Array(_)) => Item::Array(vec![Item::Value(*l)]).partial_cmp(rhs),
            (Item::Array(_), Item::Value(r)) => lhs.partial_cmp(&Item::Array(vec![Item::Value(*r)]))
        }
    }
}

fn score(pairs: &[(Item, Item)]) -> usize {
    pairs.iter()
        .enumerate()
        .map(|(index, (left, right))| {
            if left < right {
                index + 1
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let pairs = parse(include_str!("day13.txt"));
    println!("score: {}", score(&pairs));

    let mut items = pairs.into_iter().map(|(l, r)| vec![l, r]).flatten().collect_vec();
    let div_2 = Item::Array(vec![Item::Array(vec![Item::Value(2)])]);
    let div_6 = Item::Array(vec![Item::Array(vec![Item::Value(6)])]);
    items.extend(vec![div_2.clone(), div_6.clone()]);
    items.sort_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap());
    let pos2 = items.iter().position(|item| item == &div_2).unwrap();
    let pos6 = items.iter().position(|item| item == &div_6).unwrap();
    println!("decoder: {}", (pos2 + 1) * (pos6 + 1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let pairs = parse(r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]");
        assert_eq!(score(&pairs), 13);

        let mut pairs = pairs.into_iter().map(|(l, r)| vec![l, r]).flatten().collect_vec();
        pairs.sort_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap());
        println!("{:?}", pairs);
    }
}
