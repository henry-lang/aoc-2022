use std::{collections::HashSet, str::FromStr};

enum Move {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
}

impl Move {
    pub fn amount(&self) -> i64 {
        match self {
            Self::Up(n) | Self::Down(n) | Self::Left(n) | Self::Right(n) => *n,
        }
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, num) = s.split_once(' ').ok_or(())?;
        let num = num.parse().map_err(|_| ())?;

        match dir {
            "U" => Ok(Self::Up(num)),
            "D" => Ok(Self::Down(num)),
            "L" => Ok(Self::Left(num)),
            "R" => Ok(Self::Right(num)),
            _ => Err(()),
        }
    }
}

fn move_iter(input: &str) -> impl Iterator<Item = Move> + '_ {
    input.lines().flat_map(str::parse)
}

pub fn part_a(input: &str) -> impl ToString {
    let mut visited = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    for m in move_iter(input) {
        visited.insert(tail);
    }

    visited.len()
}

pub fn part_b(input: &str) -> impl ToString {
    0
}

crate::test_day!(9, 13, 0);
