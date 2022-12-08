struct Matrix {
    rows: Vec<Vec<usize>>
}

impl Matrix {
    pub fn from(input: &str) -> Self {
        let rows = input.lines()
            .map(|line| {
                line.chars()
                    .map(|c| usize::from_str_radix(&c.to_string(), 10).unwrap())
                    .collect()
            })
            .collect();

        Self {
            rows
        }
    }

    fn width(&self) -> usize {
        self.rows.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn vis_from(&self, mut pos: (i32, i32), offset: (i32, i32)) -> bool {
        let val = self.rows[pos.1 as usize][pos.0 as usize];
        loop {
            pos.0 += offset.0;
            pos.1 += offset.1;
            if pos.0 < 0 || pos.1 < 0 || pos.0 >= self.width() as i32 || pos.1 >= self.height() as i32 {
                return true;
            }
            if self.rows[pos.1 as usize][pos.0 as usize] >= val {
                return false;
            }
        }
    }

    fn score_from(&self, mut pos: (i32, i32), offset: (i32, i32)) -> usize {
        let val = self.rows[pos.1 as usize][pos.0 as usize];
        let mut cnt = 0;
        loop {
            pos.0 += offset.0;
            pos.1 += offset.1;
            if pos.0 < 0 || pos.1 < 0 || pos.0 >= self.width() as i32 || pos.1 >= self.height() as i32 {
                break;
            }

            cnt += 1;

            if self.rows[pos.1 as usize][pos.0 as usize] >= val {
                break;
            }
        }
        cnt
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        self.score_from((x as i32, y as i32), (-1, 0)) *
            self.score_from((x as i32, y as i32), (1, 0)) *
            self.score_from((x as i32, y as i32), (0, -1)) *
            self.score_from((x as i32, y as i32), (0, 1))
    }
}

fn num_visible(input: &str) -> usize {
    let mtx = Matrix::from(input);

    let mut cnt = 0;
    for y in 0..(mtx.height() as i32) {
        for x in 0..(mtx.width() as i32) {
            if mtx.vis_from((x, y), (-1, 0)) || mtx.vis_from((x, y), (1, 0)) || mtx.vis_from((x, y), (0, -1)) || mtx.vis_from((x, y), (0, 1)) {
                cnt += 1;
            }
        }
    }

    cnt
}

fn max_score(input: &str) -> usize {
    let mtx = Matrix::from(input);

    let mut score = 0;
    for y in 0..mtx.height() {
        for x in 0..mtx.width() {
            let new_score = mtx.scenic_score(x, y);
            if new_score > score {
                score = new_score;
            }
        }
    }

    score
}

fn main() {
    println!("num_visible: {}", num_visible(include_str!("day8.txt")));
    println!("max_score: {}", max_score(include_str!("day8.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r"30373
25512
65332
33549
35390";
        assert_eq!(num_visible(input), 21);

        let mtx = Matrix::from(input);
        assert_eq!(mtx.score_from((2, 1), (0, -1)), 1);
        assert_eq!(mtx.score_from((2, 1), (-1, 0)), 1);
        assert_eq!(mtx.score_from((2, 1), (1, 0)), 2);
        assert_eq!(mtx.score_from((2, 1), (0, 1)), 2);
        assert_eq!(mtx.scenic_score(2, 1), 4);

        assert_eq!(max_score(input), 8);
    }
}
