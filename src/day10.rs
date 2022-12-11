use std::str::FromStr;

enum Instruction {
    Noop,
    Add(isize),
}

impl Instruction {
    pub fn cycle_time(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::Add(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(' ');
        let mut next_word = || words.next().ok_or(());

        match next_word()? {
            "noop" => Ok(Self::Noop),
            "addx" => Ok(Self::Add(next_word()?.parse().map_err(|_| ())?)),
            _ => Err(()),
        }
    }
}

pub fn part_a(input: &str) -> impl ToString {
    let mut cycle = 0;
    let mut x = 1;
    let mut sum = 0;
    for instruction in input.lines().flat_map(|l| l.parse::<Instruction>()) {
        for _ in 0..instruction.cycle_time() {
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                sum += cycle * x;
            }
        }
        match instruction {
            Instruction::Add(n) => {
                x += n;
            }
            _ => {}
        }
    }

    sum
}

pub fn part_b(input: &str) -> impl ToString {
    let mut display = String::new();
    let mut cycle = 0;
    let mut x = 1;

    for instruction in input.lines().flat_map(|l| l.parse::<Instruction>()) {
        for _ in 0..instruction.cycle_time() {
            display += if cycle % 40 >= x - 1 && cycle % 40 <= x + 1 {
                "#"
            } else {
                "."
            };
            cycle += 1;
            if cycle % 40 == 0 && cycle != 240 {
                display += "\n";
            }
        }

        match instruction {
            Instruction::Add(n) => {
                x += n;
            }
            _ => {}
        }
    }

    display
}

crate::test_day!(
    10,
    13140,
    r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#
);
