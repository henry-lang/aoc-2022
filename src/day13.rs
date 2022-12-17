use std::{cmp::Ordering, iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Eq)]
enum Data {
    Num(u8),
    List(Vec<Self>),
}

impl Data {
    fn parse_list_values(iter: &mut Peekable<Chars>) -> Result<Vec<Self>, ()> {
        let _ = iter.next(); // Skip [

        let mut values = vec![];
        if iter.next_if(|&c| c == ']').is_some() {
            return Ok(values);
        }
        values.push(Self::parse_value(iter)?);

        while iter.next_if(|&c| c == ',').is_some() {
            values.push(Self::parse_value(iter)?);
        }

        let _ = iter.next();

        Ok(values)
    }

    fn parse_num(iter: &mut Peekable<Chars>) -> Result<Self, ()> {
        let mut num = String::new();
        while let Some(digit) = iter.next_if(|c| c.is_ascii_digit()) {
            num.push(digit);
        }

        Ok(Self::Num(num.parse::<u8>().map_err(|_| ())?))
    }

    fn parse_value(iter: &mut Peekable<Chars>) -> Result<Self, ()> {
        match iter.peek().ok_or(())? {
            '[' => Ok(Self::List(Self::parse_list_values(iter)?)),
            '0'..='9' => Ok(Self::parse_num(iter)?),
            _ => Err(()),
        }
    }

    pub fn parse(line: &str) -> Result<Self, ()> {
        Self::parse_value(&mut line.chars().peekable())
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Num(a), Self::Num(b)) => a.cmp(b),
            (a, Self::Num(b)) => a.cmp(&Self::List(vec![Self::Num(*b)])),
            (Self::Num(a), b) => Self::List(vec![Self::Num(*a)]).cmp(b),
            (Self::List(a), Self::List(b)) => {
                for i in 0..a.len().max(b.len()) {
                    match (a.get(i), b.get(i)) {
                        (Some(a), Some(b)) => {
                            if let ord @ (Ordering::Less | Ordering::Greater) = a.cmp(b) {
                                return ord;
                            }
                        }
                        (Some(_), None) => return Ordering::Greater,
                        (None, Some(_)) => return Ordering::Less,
                        _ => unreachable!(),
                    }
                }
                Ordering::Equal // Both empty
            }
        }
    }
}

fn packet_iter(input: &str) -> impl Iterator<Item = Data> + '_ {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .flat_map(Data::parse)
}

pub fn part_a(input: &str) -> impl ToString {
    packet_iter(input)
        .array_chunks::<2>()
        .zip(1..)
        .filter(|([a, b], _)| a.cmp(b) == Ordering::Less)
        .map(|(_, idx)| idx)
        .sum::<usize>()
}

pub fn part_b(input: &str) -> impl ToString {
    let divider_a = Data::List(vec![Data::List(vec![Data::Num(2)])]);
    let divider_b = Data::List(vec![Data::List(vec![Data::Num(6)])]);
    packet_iter(input)
        .map(|p| (divider_a.cmp(&p), divider_b.cmp(&p)))
        .fold([1, 2], |vals @ [index_a, index_b], ord| match ord {
            (Ordering::Greater, Ordering::Greater) => [index_a + 1, index_b + 1],
            (Ordering::Greater, _) => [index_a + 1, index_b],
            (_, Ordering::Greater) => [index_a, index_b + 1],
            _ => vals
        })
        .iter()
        .product::<usize>()
}

crate::test_day!(13, 13, 140);
