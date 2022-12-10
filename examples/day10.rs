enum Instruction {
    Addx(i32),
    Noop
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Instruction::Addx(_) => 2,
            Instruction::Noop => 1
        }
    }
}

#[derive(Default)]
struct Cpu {
    instructions: Vec<Instruction>,
    pc: usize,
    x: i32,
    cycle: usize,
    cycles_left_in_instruction: usize,
}

impl Cpu {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            cycle: 1,
            x: 1,
            ..Default::default()
        }
    }

    fn step(&mut self) -> bool {
        let instruction = &self.instructions[self.pc];
        if self.cycles_left_in_instruction == 0 {
            self.cycles_left_in_instruction = instruction.cycles();
        }

        self.cycles_left_in_instruction -= 1;
        if self.cycles_left_in_instruction == 0 {
            match instruction {
                Instruction::Addx(addx) => {
                    self.x += *addx;
                },
                _ => ()
            }

            self.pc += 1;
            if self.pc == self.instructions.len() {
                return false;
            }
        }

        self.cycle += 1;
        true
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|line| {
            if line == "noop" {
                Instruction::Noop
            } else {
                let amount = line.strip_prefix("addx ").unwrap();
                Instruction::Addx(i32::from_str_radix(amount, 10).unwrap())
            }
        })
        .collect()
}

fn calc_sum(input: &str) -> i32 {
    let mut cpu = Cpu::new(parse(input));
    let mut sum = 0;
    let interesting_cycles = vec![20, 60, 100, 140, 180, 220];
    while cpu.step() {
        if interesting_cycles.contains(&cpu.cycle) {
            //println!("cpu: {} {}", cpu.cycle, cpu.x);
            sum += cpu.cycle as i32 * cpu.x;
        }
    }
    sum
}

fn print(input: &str) {
    let mut cpu = Cpu::new(parse(input));
    let mut crt_x = 1;
    let mut pixels = "".to_string();
    loop {
        if crt_x >= cpu.x && crt_x <= cpu.x + 2 {
            pixels += "#";
        } else {
            pixels += ".";
        }
        crt_x += 1;
        if crt_x > 40 {
            println!("{}", pixels);
            pixels = "".to_string();
            crt_x = 1;
        }

        if !cpu.step() {
            break;
        }
    }
    println!("{}", pixels);
}

fn main() {
    println!("sum: {}", calc_sum(include_str!("day10.txt")));
    print(include_str!("day10.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("day10_example.txt");
        assert_eq!(calc_sum(input), 13140);
        print(input);
    }
}
