fn solve<const N: usize>(input: &str) -> usize {
    input
        .chars()
        .collect::<Vec<_>>()
        .array_windows::<N>()
        .enumerate()
        .find(|(_, a)| {
            !a.iter()
                .enumerate()
                .any(|(i, c)| a.iter().enumerate().any(|(j, cc)| c == cc && i != j))
        })
        .unwrap()
        .0
        + N
}

pub fn part_a(input: &str) -> impl ToString {
    solve::<4>(input)
}

pub fn part_b(input: &str) -> impl ToString {
    solve::<14>(input)
}

crate::test_day!(6, 7, 19);
