#![allow(dead_code)]

use itertools::Itertools;

#[derive(Debug)]
enum Child {
    Directory(Directory),
    File(File)
}

#[derive(Debug)]
struct Directory {
    name: String,
    children: Vec<Child>,
}

impl Directory {
    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            children: vec![]
        }
    }

    fn size(&self) -> usize {
        self.children.iter()
            .map(|child| {
                match child {
                    Child::Directory(dir) => {
                        dir.size()
                    },
                    Child::File(f) => {
                        f.size
                    }
                }
            })
            .sum()
    }

    fn add_child(&mut self, child: Child) {
        self.children.push(child);
    }

    fn find(&mut self, path: &[&str]) -> &mut Directory {
        println!("trying to find: {:?}", &path);
        let mut cur = self;
        for i in 0..path.len() {
            cur = cur.cd(path[i]).unwrap();
        }
        cur
    }

    fn cd(&mut self, target: impl Into<String>) -> Option<&mut Directory> {
        let target = target.into();

        for child in &mut self.children {
            match child {
                Child::Directory(dir) => {
                    if dir.name == target {
                        return Some(dir);
                    }
                },
                Child::File(_) => ()
            }
        }

        None
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize
}

fn parse(input: &str) -> Directory {
    let mut root = Directory::new("/");
    let mut cwd = vec![];

    let mut lines = input.lines().peekable();
    loop {
        match lines.next() {
            Some(instruction) => {
                if !instruction.starts_with('$') {
                    panic!();
                }
                let mut split = instruction.split_whitespace();
                let _ = split.next();
                match split.next() {
                    Some("cd") => {
                        let arg = split.next().unwrap();
                        if arg == "/" {
                            cwd = vec![];
                        } else if arg == ".." {
                            cwd.pop();
                        } else {
                            cwd.push(arg);
                        }
                    },
                    Some("ls") => {
                        let directory = root.find(&cwd);
                        loop {
                            let next = lines.peek();
                            if next.is_some() && !next.unwrap().starts_with('$') {
                                let line = lines.next().unwrap();
                                let mut split = line.split_whitespace();
                                let ty_or_size = split.next().unwrap();
                                let name = split.next().unwrap();
                                if ty_or_size == "dir" {
                                    directory.add_child(Child::Directory(Directory::new(name)));
                                } else {
                                    directory.add_child(Child::File(File {
                                        name: name.to_string(),
                                        size: usize::from_str_radix(ty_or_size, 10).unwrap(),
                                    }));
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    _ => panic!()
                }
            },
            None => break
        }
    }

    root
}

fn print(root: &Directory, indent: usize) {
    println!("{:indent$}- {} (dir, size={})", "", root.name, root.size());
    for child in &root.children {
        match child {
            Child::Directory(dir) => {
                print(dir, indent + 2);
            },
            Child::File(file) => {
                println!("  {:indent$}- {} (file, size={})", "", file.name, file.size);
            }
        }
    }
}

fn calc_small_dir_sum(root: &Directory) -> Vec<&Directory> {
    let mut result = vec![];

    for child in &root.children {
        match &child {
            Child::Directory(dir) => {
                result.extend(calc_small_dir_sum(dir));
            },
            Child::File(_) => ()
        }
    }

    if root.size() < 100000 {
        result.push(root);
    }

    result
}

fn calc_size_candidates(root: &Directory) -> Vec<usize> {
    let mut candidates = vec![];
    for child in &root.children {
        match child {
            Child::Directory(dir) => {
                candidates.extend(calc_size_candidates(dir));
            },
            Child::File(_) => ()
        }
    }
    candidates.push(root.size());

    candidates
}

pub fn main() {
    let root = parse(include_str!("day7.txt"));
    print(&root, 0);
    println!("sum: {}", calc_small_dir_sum(&root).iter().map(|d| d.size()).sum::<usize>());

    let unused = 70_000_000 - root.size();
    println!("unused: {}", 70_000_000 - root.size());
    let extra_space_required = 30_000_000 - (70_000_000 - root.size());
    println!("extra_space_required: {}",extra_space_required);
    let candidates = calc_size_candidates(&root)
        .into_iter()
        .filter(|size| {
            unused + size >= 30_000_000
        })
        .sorted()
        .collect_vec();
    println!("{:#?}", candidates);
    println!("smallest dir size: {}", candidates.first().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        let root = parse(input);
        assert_eq!(calc_small_dir_sum(&root).iter().map(|d| d.size()).sum::<usize>(), 95437);
        print(&root, 0);

        let mut candidates = calc_size_candidates(&root);
        candidates.sort();
        println!("{:#?}", candidates);
    }
}
