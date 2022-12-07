use std::{collections::HashMap, rc::Rc, str::FromStr};

enum Command<'a> {
    List,
    Cd(&'a str),
    Dir(&'a str),
    FileInfo(&'a str, usize),
}

impl FromStr for Command<'_> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let next_word = || words.next().ok_or(());
        let first = next_word()?;
        match first.chars().next().ok_or(())? {
            '0'..='9' => Ok(Self::FileInfo(
                next_word()?,
                first.parse::<usize>().map_err(|_| ())?,
            )),
            'd' => Ok(Self::Dir(next_word()?)),
            '$' => match next_word()? {
                "cd" => Ok(Self::Cd(next_word()?)),
                "ls" => Ok(Self::List),
                _ => Err(()),
            },
        }
    }
}

enum Node<'a> {
    Dir(Option<Rc<Node<'a>>>, HashMap<&'a str, Rc<Node<'a>>>),
    File(&'a str, usize),
}

fn build_fs(input: &str) -> Node {
    let mut root = Rc::new(Node::Dir(None, Default::default()));
    let mut current_node = root.clone();

    let mut commands = input
        .lines()
        .map(|l| l.parse::<Command>().expect("parse command"));

    while let Some(command) = commands.next() {
        use Command::*;
        match command {
            Cd(path) => match path {
                ".." => {
                    current_node = match current_node.as_ref() {
                        Node::Dir(parent, _) => parent.unwrap(),
                        _ => panic!(),
                    }
                    .clone()
                }
                "/" => {}
                _ => {
                    current_node = match current_node.as_ref() {
                        Node::Dir(_, children) => children,
                        _ => panic!(),
                    }
                    .get(path)
                    .unwrap()
                    .clone();
                }
            },
            List => {
                for command in commands.take
            }
        }
    }

    root
}

pub fn part_a(input: &str) -> impl ToString {
    let fs = build_fs(input);
}

pub fn part_b(input: &str) -> impl ToString {
    0
}

crate::test_day!(7, 95437, 0);
