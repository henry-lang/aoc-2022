use std::{collections::VecDeque, str::FromStr};

enum Op {
    Plus,
    Minus,
    Times,
    Divide,
}

impl Op {
    pub fn run(&self, old: u64, operand: u64) -> u64 {
        match self {
            Self::Plus => old + operand,
            Self::Minus => old - operand,
            Self::Times => old * operand,
            Self::Divide => old / operand,
        }
    }
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            "*" => Ok(Self::Times),
            "/" => Ok(Self::Divide),
            _ => Err(()),
        }
    }
}

enum Operand {
    Old,
    Number(u64),
}

impl Operand {
    pub fn value(&self, old: u64) -> u64 {
        match self {
            Self::Old => old,
            Self::Number(n) => *n,
        }
    }
}

impl FromStr for Operand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Self::Old),
            _ => Ok(Self::Number(s.parse::<u64>().map_err(|_| ())?)),
        }
    }
}

struct Monkey {
    items: VecDeque<u64>,
    operation: (Op, Operand),
    divisible_test: u64,
    if_true: usize,
    if_false: usize,
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .lines()
        .array_chunks::<7>()
        .map(|lines| Monkey {
            items: lines[1]
                .trim_start()
                .strip_prefix("Starting items: ")
                .unwrap()
                .split(", ")
                .flat_map(str::parse)
                .collect(),
            operation: lines[2]
                .trim_start()
                .strip_prefix("Operation: new = old ")
                .unwrap()
                .split_once(' ')
                .map(|(op, operand)| (op.parse().unwrap(), operand.parse().unwrap()))
                .unwrap(),
            divisible_test: lines[3]
                .split_whitespace()
                .last()
                .and_then(|n| n.parse().ok())
                .unwrap(),
            if_true: lines[4]
                .split_whitespace()
                .last()
                .and_then(|n| n.parse().ok())
                .unwrap(),
            if_false: lines[5]
                .split_whitespace()
                .last()
                .and_then(|n| n.parse().ok())
                .unwrap(),
        })
        .collect()
}

fn play(input: &str, rounds: usize, div_three: bool) -> usize {
    let mut monkeys = parse(input);
    let modulo = monkeys.iter().map(|m| m.divisible_test).product::<u64>();

    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            while let Some(item) = monkeys[m].items.pop_front() {
                inspections[m] += 1;

                let item = monkeys[m]
                    .operation
                    .0
                    .run(item, monkeys[m].operation.1.value(item))
                    % modulo
                    / if div_three { 3 } else { 1 };

                let destination = if item % monkeys[m].divisible_test == 0 {
                    monkeys[m].if_true
                } else {
                    monkeys[m].if_false
                };

                monkeys[destination].items.push_back(item)
            }
        }
    }

    inspections.sort_by(|a, b| b.cmp(a));
    inspections[0] * inspections[1]
}

pub fn part_a(input: &str) -> impl ToString {
    play(input, 20, true)
}

pub fn part_b(input: &str) -> impl ToString {
    play(input, 10_000, false)
}

crate::test_day!(11, 10_605, 2_713_310_158u64);
