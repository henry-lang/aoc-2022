use std::collections::VecDeque;

type Stack = VecDeque<char>;

fn parse(input: &str) -> (Vec<Stack>, impl Iterator<Item = (usize, usize, usize)> + '_) {
    (
        input.lines().take_while(|l| !l.starts_with(" 1")).fold(
            vec![VecDeque::new(); (input.lines().next().unwrap().len() + 1) / 4],
            |mut a, l| {
                l.chars()
                    .skip(1)
                    .step_by(4)
                    .enumerate()
                    .filter(|(_, c)| c.is_ascii_uppercase())
                    .for_each(|(i, c)| a[i].push_front(c));
                a
            },
        ),
        input
            .lines()
            .skip_while(|l| !l.starts_with('m'))
            .map(|l| {
                l.split(' ')
                    .skip(1)
                    .step_by(1)
                    .flat_map(|l| l.parse::<usize>())
            })
            .map(|mut l| (l.next().unwrap(), l.next().unwrap() - 1, l.next().unwrap() - 1)),
    )
}

pub fn part_a(input: &str) -> impl ToString {
    let (mut stacks, moves) = parse(input);

    for m in moves {
        for _ in 0..m.0 {
            let val = stacks[m.1].pop_back().unwrap();
            stacks[m.2].push_back(val);
        }
    }

    stacks.iter().map(|s| s.back().unwrap()).collect::<String>()
}

pub fn part_b(input: &str) -> impl ToString {
    let (mut stacks, moves) = parse(input);

    for m in moves {
        for i in 0..m.0 {
            let len = stacks[m.1].len();
            let val = stacks[m.1][len - m.0 + i];
            stacks[m.2].push_back(val);
        }

        for _ in 0..m.0 {
            stacks[m.1].pop_back().unwrap();
        }
    }

    stacks.iter().map(|s| s.back().unwrap()).collect::<String>()
}

crate::test_day!(5, "CMZ", "MCD");
