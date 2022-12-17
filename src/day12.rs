use std::collections::{HashSet, VecDeque};

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn parse_grid(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let grid = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let mut start = (0, 0);
    'outer: for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'E' {
                start = (i as usize, j as usize);
                break 'outer;
            }
        }
    }

    (grid, start)
}

fn depth(c: char) -> u8 {
    match c {
        'a'..='z' => c as u8 - b'a',
        'S' => 0,
        'E' => 25,
        _ => panic!(),
    }
}

struct State {
    pos: (usize, usize),
    steps: usize,
}

fn solve(input: &str, target: char) -> usize {
    let (grid, start) = parse_grid(input);

    let mut queue = VecDeque::new();
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut best = usize::MAX;
    queue.push_back(State {
        pos: start,
        steps: 0,
    });

    while let Some(State {
        pos: current @ (x, y),
        steps,
    }) = queue.pop_front()
    {
        if visited.contains(&current) {
            continue;
        }


        let current_depth = depth(grid[x][y]);

        if grid[x][y] == target {
            best = best.min(steps);
            break;
        }

        for pos in std::iter::repeat(current)
            .zip(DIRS.iter())
            .map(|(a, b)| (a.0 as isize + b.0, a.1 as isize + b.1))
            .filter(|&(a, b)| {
                a >= 0 && a < grid.len() as isize && b >= 0 && b < grid[0].len() as isize
            })
            .map(|(a, b)| (a as usize, b as usize))
        {
            let next_depth = depth(grid[pos.0][pos.1]);
            if next_depth < current_depth.checked_sub(1).unwrap_or(0) || visited.contains(&pos) {
                continue;
            }

            queue.push_back(State {pos, steps: steps + 1});
        }
        visited.insert(current);
    }

    best
}

pub fn part_a(input: &str) -> impl ToString {
    solve(input, 'S')
}

pub fn part_b(input: &str) -> impl ToString {
    solve(input, 'a')
}

crate::test_day!(12, 31, 29);
