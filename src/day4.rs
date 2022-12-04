fn pairs_iter(input: &str) -> impl Iterator<Item = [u64; 4]> + '_ {
    input.lines().map(|p| {
        p.split(|c| c == ',' || c == '-')
            .array_chunks::<4>()
            .next()
            .unwrap()
            .map(|n| n.parse::<u64>().unwrap())
    })
}

pub fn part_a(input: &str) -> impl ToString {
    pairs_iter(input)
        .filter(|[a, b, c, d]| a <= c && b >= d || (c <= a) && (d >= b))
        .count()
}

pub fn part_b(input: &str) -> impl ToString {
    pairs_iter(input)
        .filter(|[a, b, c, d]| a <= d && b >= c)
        .count()
}

crate::test_day!(4, 2, 4);
