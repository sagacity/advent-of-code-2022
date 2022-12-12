use std::collections::HashMap;
use glam::IVec2;
use itertools::Itertools;
use petgraph::algo::astar;
use petgraph::prelude::*;

fn parse(input: &str, include_all_starts: bool) -> (Graph<IVec2, (), Directed>, Vec<NodeIndex>, NodeIndex) {
    let mut heights = vec![];
    let mut g = Graph::new();
    let mut starts = vec![];
    let mut end = None;
    let mut nxs = HashMap::new();
    let mut max = IVec2::ZERO;
    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            let height = match c {
                'S' => {
                    0
                },
                'E' => {
                    25
                },
                _ => c as u8 - 'a' as u8
            };
            row.push(height);

            let pos = IVec2::new(x as i32, y as i32);
            max = max.max(pos);

            let nx = g.add_node(pos);
            nxs.insert(pos, nx);

            if c == 'S' || (include_all_starts && c == 'a') {
                starts.push(nx);
            }
            if c == 'E' {
                end = Some(nx);
            }
        }
        heights.push(row);
    }

    for y in 0..heights.len() {
        let row = &heights[y];
        for x in 0..row.len() {
            let pos = IVec2::new(x as i32, y as i32);
            let nx = nxs.get(&pos).unwrap();
            let height = heights[y][x] as i32;

            let get_nx = |offset: IVec2| {
                let other_pos = pos + offset;
                if other_pos.x < 0 || other_pos.y < 0 || other_pos.x > max.x || other_pos.y > max.y {
                    return None;
                }
                let other_height = heights[other_pos.y as usize][other_pos.x as usize] as i32;
                if other_height <= height + 1 {
                    Some(nxs.get(&other_pos).unwrap())
                } else {
                    None
                }
            };
            get_nx(IVec2::new(-1, 0)).map(|other_nx| g.update_edge(*nx, *other_nx, ()));
            get_nx(IVec2::new(1, 0)).map(|other_nx| g.update_edge(*nx, *other_nx, ()));
            get_nx(IVec2::new(0, -1)).map(|other_nx| g.update_edge(*nx, *other_nx, ()));
            get_nx(IVec2::new(0, 1)).map(|other_nx| g.update_edge(*nx, *other_nx, ()));
        }
    }

    //println!("{:#?}", g);
    (g, starts, end.unwrap())
}

fn main() {
    let (graph, starts, end) = parse(include_str!("day12.txt"), false);
    let path = astar(&graph, *starts.first().unwrap(), |finish| finish == end, |_| 1, |_| 0);
    println!("num steps: {}", path.unwrap().0);

    let (graph, starts, end) = parse(include_str!("day12.txt"), true);
    let min_steps = starts.into_iter().filter_map(|start| {
        let path = astar(&graph, start, |finish| finish == end, |_| 1, |_| 0);
        path.map(|(len, _)| len)
    }).min();
    println!("num steps: {}", min_steps.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let (graph, start, end) = parse(input);
        let path = astar(&graph, start, |finish| finish == end, |_| 1, |_| 0);
        assert_eq!(path.unwrap().0, 31);
    }
}
