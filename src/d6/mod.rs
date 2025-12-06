#[derive(Debug)]
enum Operator {
    Addition,
    Multiplication,
}

#[derive(Debug, Clone, Copy)]
enum Alignment {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct NumAligned<'a> {
    align: Alignment,
    str: &'a str,
}

impl<'a> From<&NumAligned<'a>> for u64 {
    fn from(val: &NumAligned<'a>) -> Self {
        val.str.trim().parse().unwrap()
    }
}

#[derive(Debug)]
struct Col<'a> {
    op: Operator,
    numbers: Vec<NumAligned<'a>>,
}

impl<'a> Col<'a> {
    fn numbers_aligned(&self) -> Vec<u64> {
        let longest = self.numbers.iter().map(|n| n.str.len()).max().unwrap();
        (0..longest)
            .map(|n| self.num_at_pos_aligned(n, longest))
            .collect()
    }
    fn num_at_pos_aligned(&self, pos: usize, longest: usize) -> u64 {
        let numbers: Vec<_> = self
            .numbers
            .iter()
            .filter_map(|num_str| {
                let adjusted_pos = match num_str.align {
                    Alignment::Left => pos,
                    Alignment::Right => {
                        let cur_len = num_str.str.len();
                        let pad = longest - cur_len;

                        if pos < pad || pos >= pad + cur_len {
                            return None;
                        }

                        pos - pad
                    }
                };
                num_str.str.as_bytes().get(adjusted_pos)
            })
            .map(|b| b - b'0') // convert byte to number
            .collect();

        let mut n = 0u64;
        for c in numbers {
            n = n * 10 + c as u64;
        }
        n
    }
}

pub fn solve() {
    let input = include_str!("./input.txt");
    let mut lines: Vec<&str> = input.split_terminator('\n').collect();

    let operators = lines.pop().unwrap();
    let split_idx: Vec<_> = operators
        .bytes()
        .enumerate()
        .filter_map(|(idx, b)| {
            if b.is_ascii_whitespace() {
                None
            } else {
                Some(idx)
            }
        })
        .collect();

    let lines: Vec<Vec<NumAligned>> = lines
        .iter()
        .map(|line| {
            split_idx
                .iter()
                .enumerate()
                .map(|(idx, start_idx)| {
                    let next_idx = split_idx.get(idx + 1);
                    let str = {
                        if let Some(next_idx) = next_idx {
                            &line[*start_idx..*next_idx]
                        } else {
                            &line[*start_idx..]
                        }
                    };
                    let align = if str.starts_with(' ') {
                        Alignment::Right
                    } else {
                        Alignment::Left
                    };
                    NumAligned {
                        align,
                        str: str.trim(),
                    }
                })
                .collect()
        })
        .collect();

    let operators: Vec<_> = operators.split_whitespace().collect();
    let mut cols: Vec<Col> = Vec::with_capacity(operators.len());

    for (idx, operator) in operators.iter().enumerate() {
        let numbers: Vec<NumAligned> = lines.iter().map(|line| line[idx]).collect();
        cols.push(Col {
            op: match *operator {
                "+" => Operator::Addition,
                "*" => Operator::Multiplication,
                _ => panic!("invalid input"),
            },
            numbers,
        });
    }

    // P1
    let mut total = 0;
    for col in cols.iter() {
        total += col.numbers.iter().fold(
            match col.op {
                Operator::Multiplication => 1,
                Operator::Addition => 0,
            },
            |acc, cur| match col.op {
                Operator::Addition => acc + Into::<u64>::into(cur),
                Operator::Multiplication => acc * Into::<u64>::into(cur),
            },
        );
    }
    println!("P1: {total}");

    // P2
    total = 0;
    for col in cols.iter() {
        total += col.numbers_aligned().iter().fold(
            match col.op {
                Operator::Addition => 0,
                Operator::Multiplication => 1,
            },
            |acc, cur| match col.op {
                Operator::Addition => acc + cur,
                Operator::Multiplication => acc * cur,
            },
        );
    }
    println!("P2: {total}");
}
