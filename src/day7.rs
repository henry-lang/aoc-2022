enum Command<'a> {
    List,
    Cd(&'a str),
    Dir(&'a str),
    FileInfo(&'a str, usize),
}

impl<'a> TryFrom<&'a str> for Command<'a> {
    type Error = ();

    fn try_from(s: &'a str) -> Result<Command<'a>, Self::Error> {
        let mut words = s.split_whitespace();
        let mut next_word = || words.next().ok_or(());
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
            _ => panic!(),
        }
    }
}

struct Node {
    size: usize, // Enables caching in immutable struct
    children: Vec<Node>,
}

impl Node {
    pub fn traverse_dirs(&self) -> Box<dyn Iterator<Item = &Node> + '_> {
        Box::new(
            self.children
                .iter()
                .filter(|c| !c.children.is_empty())
                .flat_map(|c| c.traverse_dirs())
                .chain(std::iter::once(self)),
        )
    }

    pub fn add_child(&mut self, child: Node) {
        self.size += child.size;
        self.children.push(child);
    }

    pub fn build<'a>(mut self, commands: &mut impl Iterator<Item = Command<'a>>) -> Self {
        while let Some(command) = commands.next() {
            use Command::*;
            match command {
                Cd(path) => match path {
                    ".." => {
                        break; // We've exited out of this directory so we're done
                    }
                    _ => self.add_child(
                        Node {
                            size: 0,
                            children: Vec::new(),
                        }
                        .build(commands),
                    ),
                },
                FileInfo(_, size) => {
                    self.add_child(Node {
                        children: Vec::new(),
                        size,
                    });
                }
                _ => {}
            }
        }

        self
    }
}

fn build_fs(input: &str) -> Node {
    let mut commands = input
        .lines()
        .map(|l| Command::try_from(l).expect("parse command"))
        .peekable();

    Node {
        size: 0,
        children: Vec::new(),
    }
    .build(&mut commands)
}

pub fn part_a(input: &str) -> impl ToString {
    build_fs(input)
        .traverse_dirs()
        .map(|d| d.size)
        .filter(|&size| size <= 100_000)
        .sum::<usize>()
}

pub fn part_b(input: &str) -> impl ToString {
    let fs = build_fs(input);
    let needed = 30_000_000 - (70_000_000 - fs.size);

    fs.traverse_dirs()
        .map(|d| d.size)
        .filter(|&size| size >= needed)
        .min()
        .unwrap_or(0)
}

crate::test_day!(7, 95437, 24933642);
