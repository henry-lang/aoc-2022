fn elf_iter(input: &str) -> impl Iterator<Item = u64> + '_ {
    input
        .split("\n\n")
        .map(|elf| elf.lines().flat_map(|n| n.parse::<u64>()).sum())
}

pub fn part_a(input: &str) -> impl ToString {
    elf_iter(input).max().unwrap()
}

pub fn part_b(input: &str) -> impl ToString {
    let mut elves = elf_iter(input).collect::<Vec<_>>();
    elves.sort_by(|a, b| b.cmp(a)); // Reverse sort
    elves.iter().take(3).sum::<u64>()
}

crate::test_day!(1, 24000, 45000);
