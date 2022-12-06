use std::collections::VecDeque;
use std::collections::BTreeSet;

fn find_markers(input: &str, window_size: usize) -> Vec<usize> {
    let mut window = VecDeque::new();
    let mut markers = vec![];

    for (index, char) in input.chars().enumerate() {
        window.push_front(char);
        if window.len() > window_size {
            window.pop_back();
        }

        if window.iter().collect::<BTreeSet<_>>().len() == window_size {
            markers.push(index + 1);
        }
    }

    markers
}

pub fn main() {
    println!("markers: {:?}", find_markers(include_str!("day6.txt"), 4));
    println!("messages: {:?}", find_markers(include_str!("day6.txt"), 14));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(*find_markers("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).first().unwrap(), 5);
        assert_eq!(*find_markers("nppdvjthqldpwncqszvftbrmjlhg", 4).first().unwrap(), 6);
        assert_eq!(*find_markers("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4).first().unwrap(), 10);
        assert_eq!(*find_markers("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4).first().unwrap(), 11);
        assert_eq!(*find_markers("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).first().unwrap(), 19);
    }
}
