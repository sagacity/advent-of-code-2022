use itertools::Itertools;

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<usize>,
    op: Op,
    operand: Operand,
    test_divisible_by: usize,
    if_true: usize,
    if_false: usize,
    num_inspections: usize,
}

#[derive(Clone, Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Clone, Debug)]
enum Operand {
    Imm(usize),
    Old,
}

#[derive(Debug)]
struct State {
    monkeys: Vec<Monkey>,
    round: usize,
}

fn parse(input: &str) -> State {
    let mut monkeys = vec![];
    for monkey_input in input.split("\n\n") {
        let mut lines = monkey_input.lines();

        let _monkey_id = usize::from_str_radix(lines.next().unwrap().trim().strip_prefix("Monkey ").unwrap().strip_suffix(":").unwrap(), 10).unwrap();
        let items = lines.next().unwrap().trim().strip_prefix("Starting items: ").unwrap().split(",").map(|item_id| {
            usize::from_str_radix(item_id.trim(), 10).unwrap()
        }).collect_vec();
        let mut operation_parts = lines.next().unwrap().trim().strip_prefix("Operation: new = old ").unwrap().split(" ");

        let op = operation_parts.next().unwrap();
        let op = match op {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!("wtf")
        };
        let operand = operation_parts.next().unwrap();
        let operand = match operand {
            "old" => Operand::Old,
            _ => Operand::Imm(usize::from_str_radix(operand, 10).unwrap())
        };

        let test_divisible_by = usize::from_str_radix(lines.next().unwrap().trim().strip_prefix("Test: divisible by ").unwrap(), 10).unwrap();
        let if_true = usize::from_str_radix(lines.next().unwrap().trim().strip_prefix("If true: throw to monkey ").unwrap(), 10).unwrap();
        let if_false = usize::from_str_radix(lines.next().unwrap().trim().strip_prefix("If false: throw to monkey ").unwrap(), 10).unwrap();

        monkeys.push(Monkey {
            //monkey_id,
            items,
            op,
            operand,
            test_divisible_by,
            if_true,
            if_false,
            num_inspections: 0
        })
    }

    State {
        monkeys,
        round: 0
    }
}

impl State {
    fn round(&mut self, divider: usize) {
        self.round += 1;
        for monkey_id in 0..self.monkeys.len() {
            let items = std::mem::replace(&mut self.monkeys[monkey_id].items, vec![]);
            let num_items = items.len();
            for item in items {
                let monkey = self.monkeys[monkey_id].clone();
                let old_worry_level = item;
                let new_worry_level = match (&monkey.op, &monkey.operand) {
                    (Op::Add, Operand::Imm(val)) => old_worry_level + *val,
                    (Op::Add, Operand::Old) => old_worry_level * 2,
                    (Op::Mul, Operand::Imm(val)) => old_worry_level * *val,
                    (Op::Mul, Operand::Old) => old_worry_level * old_worry_level,
                } / divider;
                let new_worry_level = new_worry_level % 9_699_690;
                if new_worry_level % monkey.test_divisible_by == 0 {
                    self.monkeys[monkey.if_true].items.push(new_worry_level);
                } else {
                    self.monkeys[monkey.if_false].items.push(new_worry_level);
                }
            }

            self.monkeys[monkey_id].num_inspections += num_items;
        }
    }

    fn monkey_business(&self) -> usize {
        let top = self.monkeys.iter().map(|m| m.num_inspections).sorted().rev().take(2).collect_vec();
        top[0] * top[1]
    }
}

fn main() {
    let mut state = parse(include_str!("day11.txt"));
    for _ in 0..20 {
        state.round(3);
    }
    println!("monkey business: {}", state.monkey_business());

    let mut state = parse(include_str!("day11.txt"));
    for _ in 0..10000 {
        state.round(1);
    }
    println!("more monkey business: {}", state.monkey_business());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;
        let mut state = parse(input);
        for _ in 0..20 {
            state.round();
        }
        println!("{:#?}", state);
        println!("monkey business: {}", state.monkey_business());
    }
}
