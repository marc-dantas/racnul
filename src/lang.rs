pub enum Op {
    Inc(usize, char),
    Nxt(usize, char),
    Lst(usize, char),
    New(usize, char),
    Cpy(usize, char),
    Pst(usize, char),
    Zro(usize, char),
}

pub type Ops = Vec<Op>;

pub fn parse(source: &String) -> Ops {
    let cs = source.chars();
    let mut ops = Vec::new();
    for (i, chr) in cs.enumerate() {
        match chr {
            '+' => ops.push(Op::Inc(i, '+')),
            '>' => ops.push(Op::Nxt(i, '>')),
            '<' => ops.push(Op::Lst(i, '<')),
            '#' => ops.push(Op::New(i, '#')),
            ',' => ops.push(Op::Cpy(i, ',')),
            '.' => ops.push(Op::Pst(i, '.')),
            '%' => ops.push(Op::Zro(i, '%')),
            _ => { }
        }
    }
    return ops;
}

pub fn error(origin: &str, pos: usize, character: char, msg: &str) -> String {
    format!("error at {} at position {} ('{}'): {}", origin, pos, character, msg)
}

pub struct Executor {
    pub ops: Ops,
    pub file: String,
    pub target: Vec<u8>,
    pub cursor: usize,
}

impl Executor {
    pub fn new(ops: Ops, file: String) -> Self {
        Self {
            ops, file, target: vec![0], cursor: 0,   
        }
    }

    pub fn run(&mut self) -> Result<String, String> {
        let mut clipboard: u8 = 0;
        for op in self.ops.iter() {
            match op {
                Op::Inc(_, _) => {
                    self.target[self.cursor] += 1;
                    if self.target[self.cursor] >= 10 {
                        self.target[self.cursor] = 0;
                    }
                },
                Op::Nxt(i, c) => {
                    self.cursor += 1;
                    if self.cursor >= self.target.len() {
                        self.cursor -= 1;
                        return Err(error(&self.file, *i, *c, "Can't advance cursor since it's in the highest digit."));
                    }
                },
                Op::Lst(i, c) => {
                    if self.cursor > 0 {
                        self.cursor -= 1;
                    } else {
                        return Err(error(&self.file, *i, *c, "Can't retreat cursor since it's in the lowest digit."));
                    }
                },
                Op::New(_, _) => { self.target.push(0) },
                Op::Cpy(_, _) => { clipboard = self.target[self.cursor] },
                Op::Pst(_, _) => { self.target[self.cursor] = clipboard },
                Op::Zro(_, _) => { self.target[self.cursor] = 0 },
            }
        }
        let mut res = String::new();
        for i in self.target.iter() {
            res.push_str(&i.to_string());
        }
        return Ok(res);
    }
}