use std::rc::Rc;

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

struct DirIter {
    pos: (isize, isize),
    delta: (isize, isize),
}

impl Iterator for DirIter {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.pos;
        let (delta_x, delta_y) = self.delta;
        self.pos = (x + delta_x, y + delta_y);
        Some(self.pos)
    }
}

trait Dir {
    fn dir(self, delta: (isize, isize)) -> DirIter;
}

impl Dir for (isize, isize) {
    fn dir(self, delta: (isize, isize)) -> DirIter {
        DirIter { pos: self, delta }
    }
}

fn view_iter(map: &Vec<Vec<u8>>) -> impl Iterator<Item = [Option<usize>; 4]> + '_ {
    let map = Rc::new(map);

    map.iter().zip(0isize..).flat_map(move |(r, i)| {
        let map_clone = map.clone();
        r.iter().zip(0isize..).map(move |(c, j)| {
            let map_clone = map_clone.clone();
            DIRS.map(|d| {
                (i, j)
                    .dir(d)
                    .take_while(|(x, y)| {
                        *x >= 0
                            && *y >= 0
                            && *x < map_clone.len() as isize
                            && *y < map_clone.len() as isize
                    })
                    .enumerate()
                    .find(|(_, (x, y))| map_clone[*x as usize][*y as usize] >= *c)
                    .map(|(i, _)| i)
            })
        })
    })
}

pub fn part_a(input: &str) -> impl ToString {
    let map = input
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - b'0').collect())
        .collect::<Vec<Vec<_>>>();

    view_iter(&map).filter(|arr| arr.contains(&None)).count()
}

pub fn part_b(input: &str) -> impl ToString {
    let map = input
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - b'0').collect())
        .collect::<Vec<Vec<_>>>();

    view_iter(&map)
        .map(|arr| arr.iter().flat_map(|&d| d).product::<usize>())
        .max()
        .unwrap()
}

crate::test_day!(8, 21, 3);
