use std::collections::HashSet;

fn priority(item: char) -> u64 {
    item as u64
        - if item.is_ascii_lowercase() {
            b'a' - 1
        } else {
            b'A' - 27
        } as u64
}

pub fn part_a(input: &str) -> impl ToString {
    input
        .lines()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(a, b)| {
            *a.chars()
                .collect::<HashSet<_>>()
                .intersection(&b.chars().collect::<HashSet<_>>())
                .next()
                .unwrap()
        })
        .map(priority)
        .sum::<u64>()
}

pub fn part_b(input: &str) -> impl ToString {
    input
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            *a.chars()
                .collect::<HashSet<_>>()
                .intersection(&b.chars().collect())
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&c.chars().collect())
                .next()
                .unwrap()
        })
        .map(priority)
        .sum::<u64>()
}

crate::test_day!(3, 157, 70);
