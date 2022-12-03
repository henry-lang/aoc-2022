use std::str::FromStr;

#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn score(self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn outcome(self, other: Self) -> Outcome {
        use Outcome::*;
        use Shape::*;
        match (self, other) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Lose,
            (Rock, Scissors) => Win,
            (Paper, Rock) => Win,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Lose,
            (Scissors, Rock) => Lose,
            (Scissors, Paper) => Win,
            (Scissors, Scissors) => Draw,
        }
    }

    pub fn response(self, outcome: Outcome) -> Self {
        use Outcome::*;
        use Shape::*;
        match (self, outcome) {
            (Rock, Win) => Paper,
            (Rock, Draw) => Rock,
            (Rock, Lose) => Scissors,
            (Paper, Win) => Scissors,
            (Paper, Draw) => Paper,
            (Paper, Lose) => Rock,
            (Scissors, Win) => Rock,
            (Scissors, Draw) => Scissors,
            (Scissors, Lose) => Paper,
        }
    }
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    pub fn score(self) -> u64 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

fn round_iter<T: FromStr>(input: &str) -> impl Iterator<Item = (Shape, T)> + '_ {
    input
        .lines()
        .flat_map(|r| r.split_once(' '))
        .filter_map(|(op, me)| Some((op.parse().ok()?, me.parse::<T>().ok()?)))
}

pub fn part_a(input: &str) -> impl ToString {
    round_iter::<Shape>(input)
        .map(|(op, me)| me.outcome(op).score() + me.score())
        .sum::<u64>()
}

pub fn part_b(input: &str) -> impl ToString {
    round_iter::<Outcome>(input)
        .map(|(op, out)| out.score() + op.response(out).score())
        .sum::<u64>()
}

crate::test_day!(2, 15, 12);
