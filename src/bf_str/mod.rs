use std::{
    collections::HashMap,
    io::{self, Write},
    sync::LazyLock,
};

#[derive(Debug, Clone)]
enum Op {
    Inc,
    Dec,
    Acp,
    Out,
    Fwd,
    Bwd,
    Jpf,
    Jpb,
}

static OP_MAP: LazyLock<HashMap<char, Op>> = LazyLock::new(|| {
    HashMap::from([
        ('+', Op::Inc),
        ('-', Op::Dec),
        (',', Op::Acp),
        ('.', Op::Out),
        ('>', Op::Fwd),
        ('<', Op::Bwd),
        ('[', Op::Jpf),
        (']', Op::Jpb),
    ])
});

#[derive(Debug)]
struct Operation {
    operator: Op,
    step: usize,
}

#[derive(Debug)]
pub struct BfStr {
    ops: Vec<Operation>,
}

impl Default for BfStr {
    fn default() -> Self {
        return BfStr {
            ops: Vec::with_capacity(1024),
        };
    }
}

fn count_step(chars: &[char], to_match: &char) -> usize {
    let mut count: usize = 0;
    for c in chars {
        if c == to_match {
            count += 1;
        } else {
            break;
        }
    }
    count
}

impl From<&str> for BfStr {
    fn from(text: &str) -> Self {
        let mut bf_str = BfStr::default();
        let chars: Vec<char> = text.chars().collect();
        let mut pos: usize = 0;

        let mut jmp_stack: Vec<usize> = Vec::new();
        while pos < chars.len() {
            let c = chars[pos];
            match c {
                '+' | '-' | ',' | '.' | '<' | '>' => {
                    let op = Operation {
                        operator: OP_MAP.get(&c).unwrap().clone(),
                        step: count_step(&chars[pos..], &c),
                    };
                    pos += op.step;
                    bf_str.ops.push(op);
                }
                '[' => {
                    let op = Operation {
                        operator: OP_MAP.get(&c).unwrap().clone(),
                        step: 0,
                    };
                    pos += 1;
                    bf_str.ops.push(op);
                    jmp_stack.push(bf_str.ops.len() - 1);
                }
                ']' => match jmp_stack.pop() {
                    Some(idx) => {
                        let op = Operation {
                            operator: OP_MAP.get(&c).unwrap().clone(),
                            step: idx + 1,
                        };
                        pos += 1;
                        bf_str.ops.push(op);
                        bf_str.ops[idx].step = bf_str.ops.len();
                    }
                    None => panic!("Unbalance jump!"),
                },
                _ => pos += 1,
            }
        }
        bf_str
    }
}

impl BfStr {
    pub fn interpret(&self) {
        let mut tape: Vec<u8> = vec![0; 1024000];
        let mut prt: usize = 0;
        let mut pos: usize = 0;
        while pos < self.ops.len() {
            let op = &self.ops[pos];
            match op.operator {
                Op::Inc => {
                    let (ret, _) = tape[prt].overflowing_add(op.step as u8);
                    tape[prt] = ret;
                    pos += 1;
                }
                Op::Dec => {
                    let (ret, _) = tape[prt].overflowing_sub(op.step as u8);
                    tape[prt] = ret;
                    pos += 1;
                }
                Op::Fwd => {
                    prt += op.step;
                    while prt >= tape.len() {
                        tape.push(0);
                    }
                    pos += 1;
                }
                Op::Bwd => {
                    match prt.checked_sub(op.step) {
                        Some(ret) => prt = ret,
                        None => panic!("Tape underflow!"),
                    }
                    pos += 1;
                }
                Op::Jpf => {
                    if tape[prt] == 0 {
                        pos = op.step;
                    } else {
                        pos += 1;
                    }
                }
                Op::Jpb => {
                    if tape[prt] != 0 {
                        pos = op.step;
                    } else {
                        pos += 1;
                    }
                }
                Op::Out => {
                    for _ in 0..op.step {
                        print!("{}", char::from(tape[prt]));
                    }
                    pos += 1;
                }
                Op::Acp => {
                    print!("Pleas input a number (0-255) or a ascii char: ");
                    let _ = io::stdout().flush();
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read your input!");
                    match input.trim().parse::<u8>() {
                        Ok(num) => {
                            tape[prt] = num;
                        }
                        Err(_) => match input.chars().next() {
                            Some(c) => match c.is_ascii() {
                                true => tape[prt] = c as u8,
                                false => panic!("Invalid byte data: {}", c),
                            },
                            None => panic!("Failed to read your input"),
                        },
                    }
                    pos += 1;
                }
            }
        }
    }
}
