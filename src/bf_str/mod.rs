use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read, Write},
    path::Path,
    sync::LazyLock,
};

#[derive(Debug, Clone, PartialEq)]
enum Op {
    Upd,
    Acp,
    Out,
    Shf,
    Jpf,
    Jpb,
    Set,
    // Mov,
    Mul,
    Add,
}

static OP_MAP: LazyLock<HashMap<char, Op>> = LazyLock::new(|| {
    HashMap::from([
        ('+', Op::Upd),
        ('-', Op::Upd),
        (',', Op::Acp),
        ('.', Op::Out),
        ('>', Op::Shf),
        ('<', Op::Shf),
        ('[', Op::Jpf),
        (']', Op::Jpb),
    ])
});

#[derive(Debug, Clone, PartialEq)]
struct Operation {
    operator: Op,
    operand: isize,
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

fn count_step(chars: &[char], to_match: &char) -> (usize, usize) {
    let mut count = 0;
    let mut empty = 0;
    for c in chars {
        if c == to_match {
            count += 1;
        } else if !OP_MAP.contains_key(c) {
            empty += 1;
        } else {
            break;
        }
    }
    (count, empty)
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
                '+' | ',' | '.' | '>' => {
                    let (operand, empty) = count_step(&chars[pos..], &c);
                    let op = Operation {
                        operator: OP_MAP[&c].clone(),
                        operand: operand as isize,
                    };
                    pos += operand;
                    pos += empty;
                    bf_str.ops.push(op);
                }
                '-' | '<' => {
                    let (operand, empty) = count_step(&chars[pos..], &c);
                    let op = Operation {
                        operator: OP_MAP[&c].clone(),
                        operand: -(operand as isize),
                    };
                    pos += operand;
                    pos += empty;
                    bf_str.ops.push(op);
                }
                '[' => {
                    let op = Operation {
                        operator: OP_MAP[&c].clone(),
                        operand: 0,
                    };
                    pos += 1;
                    bf_str.ops.push(op);
                    jmp_stack.push(bf_str.ops.len() - 1);
                }
                ']' => match jmp_stack.pop() {
                    Some(idx) => {
                        let op = Operation {
                            operator: OP_MAP[&c].clone(),
                            operand: (idx + 1) as isize,
                        };
                        pos += 1;
                        bf_str.ops.push(op);
                        bf_str.ops[idx].operand = bf_str.ops.len() as isize;
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
    pub fn from_file(path: &Path) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut source = String::new();
        file.read_to_string(&mut source)?;
        Ok(BfStr::from(source.as_str()))
    }

    pub fn interpret(&self) {
        self._interpret(io::stdin(), io::stdout());
    }

    fn _interpret(&self, mut read: impl Read, mut write: impl Write) {
        let mut tape: Vec<u8> = vec![0; 1024000];
        let mut prt: usize = 0;
        let mut pos: usize = 0;
        while pos < self.ops.len() {
            let op = &self.ops[pos];
            match op.operator {
                Op::Upd => {
                    tape[prt] = (tape[prt] as isize + op.operand) as u8;
                    pos += 1;
                }
                Op::Shf => {
                    if op.operand > 0 {
                        prt += op.operand as usize;
                        while prt >= tape.len() {
                            tape.push(0);
                        }
                    } else {
                        let ret = (-op.operand) as usize;
                        if prt < ret {
                            panic!("Tape underflow!");
                        }
                        prt -= ret;
                    }
                    pos += 1;
                }
                Op::Jpf => {
                    if tape[prt] == 0 {
                        pos = op.operand as usize;
                    } else {
                        pos += 1;
                    }
                }
                Op::Jpb => {
                    if tape[prt] != 0 {
                        pos = op.operand as usize;
                    } else {
                        pos += 1;
                    }
                }
                Op::Out => {
                    for _ in 0..op.operand {
                        write!(write, "{}", char::from(tape[prt])).unwrap();
                    }
                    pos += 1;
                }
                Op::Acp => {
                    let mut buf = [0; 1];
                    let _ = read.read_exact(&mut buf);
                    if buf[0] != 0 {
                        tape[prt] = buf[0];
                    }
                    pos += 1;
                }
                Op::Set => {
                    tape[prt] = op.operand as u8;
                    pos += 1;
                }
                Op::Mul => {
                    tape[prt] = (op.operand * tape[prt] as isize) as u8;
                    pos += 1;
                }
                // Op::Mov => {
                //     tape[prt + op.operand] = tape[prt];
                //     tape[prt] = 0;
                //     pos += 1;
                // }
                Op::Add => {
                    // add current value to relative operand cell
                    let new_prt;
                    if op.operand > 0 {
                        new_prt = prt + op.operand as usize;
                        while new_prt >= tape.len() {
                            tape.push(0);
                        }
                    } else {
                        let ret = (-op.operand) as usize;
                        if prt < ret {
                            panic!("Tape underflow!");
                        }
                        new_prt = prt - ret;
                    }
                    tape[new_prt] += tape[prt];
                    pos += 1;
                }
            }
        }
    }
}

static FILE_HEAD: &str = concat!(
    "#include <assert.h>\n",
    "#include <stdbool.h>\n",
    "#include <inttypes.h>\n",
    "#include <stdint.h>\n",
    "#include <stdio.h>\n",
    "#include <stdlib.h>\n",
    "\n",
    "#define CAP 1024\n",
    "\n",
    "#define da_append(da, item)                                                         \\\n",
    "    do {                                                                            \\\n",
    "        if ((da)->len >= (da)->cap) {                                               \\\n",
    "            (da)->cap = ((da)->cap == 0) ? CAP : (da)->cap * 2;                     \\\n",
    "            (da)->items = realloc((da)->items, sizeof(*(da)->items) * (da)->cap);   \\\n",
    "            assert((da)->items != NULL && \"OOM!\");                                  \\\n",
    "        }                                                                           \\\n",
    "        (da)->items[(da)->len++] = (item);                                          \\\n",
    "    } while (0)                                                                     \\\n",
    "\n",
    "typedef struct {\n",
    "    uint8_t* items;\n",
    "    size_t len;\n",
    "    size_t cap;\n",
    "    size_t ptr;\n",
    "} Tape;\n",
    "\n",
    "uint8_t tape_curr(Tape* tape) {\n",
    "    return tape->items[tape->ptr];\n",
    "}\n",
    "\n",
    "void tape_assign(Tape* tape, uint8_t u8) {\n",
    "    tape->items[tape->ptr] = u8;\n",
    "}\n",
    "\n",
    "void tape_shift(Tape* tape, int64_t delta) {\n",
    "    int64_t ret = (int64_t)tape->ptr + delta;\n",
    "    if (ret < 0) assert(0 && \"Tape Underflow!\");\n",
    "    while ((size_t)ret >= tape->len) da_append(tape, 0);\n",
    "    tape->ptr = (size_t)ret;\n",
    "}\n",
    "\n",
    "void tape_update(Tape* tape, int64_t delta) {\n",
    "    tape->items[tape->ptr] += delta;\n",
    "}\n",
    "\n",
    "#define tape_jpf(tape, dst) if (tape_curr(tape) == 0) goto dst\n",
    "#define tape_jpb(tape, dst) if (tape_curr(tape) != 0) goto dst\n",
    "\n",
    "void tape_in(Tape* tape) {\n",
    "    int c = fgetc(stdin);\n",
    "    if (c != EOF) tape_assign(tape, c);\n",
    "}\n",
    "\n",
    "void tape_out(Tape* tape, size_t step) {\n",
    "    for (size_t i = 0; i < step; ++i) {\n",
    "        printf(\"%c\", tape_curr(tape));\n",
    "    }\n",
    "}\n",
    "void tape_add(Tape* tape, size_t delta) {\n",
    "    tape->items[tape->ptr + delta] += tape_curr(tape);\n",
    "}\n",
    "\n",
    "void tape_multiple(Tape* tape, size_t step) {\n",
    "    tape_assign(tape, (uint8_t)(tape_curr(tape) * step));\n",
    "}\n",
    "\n",
    "void tape_init(Tape* tape) {\n",
    "    for (size_t i = 0; i < CAP; ++i) {\n",
    "        da_append(tape, 0);\n",
    "    }\n",
    "}\n",
    "\n"
);

static MAIN_HEAD: &str = concat!(
    "int main(void) {\n",
    "    Tape tape = { 0 };\n",
    "    tape_init(&tape); \n",
    "    \n"
);

static MAIN_TAIL: &str = concat!(
    "    \n",
    "    free(tape.items);\n",
    "    return 0;\n",
    "}\n"
);

impl BfStr {
    fn _cc(&self, mut write: impl Write, is_optimize: bool) {
        let ops: Vec<Operation>;
        if is_optimize {
            ops = self.optimize();
        } else {
            ops = self.ops.clone();
        }
        let mut cmds: Vec<String> = Vec::new();
        let mut goto_stack: Vec<(usize, &Operation)> = Vec::new();
        for (idx, op) in ops.iter().enumerate() {
            match op.operator {
                Op::Upd => cmds.push(format!("    tape_update(&tape, {});\n", op.operand)),
                Op::Shf => cmds.push(format!("    tape_shift(&tape, {});\n", op.operand)),
                Op::Acp => cmds.push(format!("    tape_in(&tape);\n")),
                Op::Out => cmds.push(format!("    tape_out(&tape, {});\n", op.operand)),
                Op::Jpf => {
                    cmds.push(String::new());
                    goto_stack.push((idx, op));
                }
                Op::Jpb => match goto_stack.pop() {
                    Some((goto_idx, goto_op)) => {
                        cmds.push(format!(
                            "    tape_jpb(&tape, jpb{});\n    jpf{}:\n",
                            op.operand, goto_op.operand
                        ));
                        cmds[goto_idx].push_str(
                            format!(
                                "    tape_jpf(&tape, jpf{});\n    jpb{}:\n",
                                goto_op.operand, op.operand
                            )
                            .as_str(),
                        );
                    }
                    None => panic!("Unbalanced jump!"),
                },
                Op::Set => cmds.push(format!("    tape_assign(&tape, {});\n", op.operand)),
                Op::Mul => cmds.push(format!("    tape_multiple(&tape, {});\n", op.operand)),
                // Op::Mov => cmds.push(format!("    tape_move(&tape, {});\n", op.operand)),
                Op::Add => cmds.push(format!("    tape_add(&tape, {});\n", op.operand)),
            }
        }
        let _ = write!(write, "{}", FILE_HEAD);
        let _ = write!(write, "{}", MAIN_HEAD);
        for cmd in &cmds {
            let _ = write!(write, "{}", cmd);
        }
        let _ = write!(write, "{}", MAIN_TAIL);
    }

    pub fn cc(&self, save_path: &Path, is_optimize: bool) {
        let file = File::create(save_path).unwrap();
        self._cc(file, is_optimize);
    }
}

impl BfStr {
    fn optimize(&self) -> Vec<Operation> {
        let mut optimized: Vec<Operation> = Vec::with_capacity(self.ops.len());
        let mut pos = 0 as usize;
        while pos < self.ops.len() {
            let op = &self.ops[pos];
            match op.operator {
                /*
                i know where it jumps to, so i could get the loop sequence
                then i check the pattern:
                    if the len of loop sequence is one, and is `-`, that is a reset
                    if the loop sequence match [(>/<)N (+)M (</>)N -], that is multiple current value with M and plus the value in next N cell,
                        then move that value to next N cell
                            to check the pattern inside of the loop, it suppose to have Op::Shf(±N) Op::Upd(M) Op::Bwd(∓N) Op::Upd(-1)
                            then these instructions along with the jumps, can be replace with: 1) Mul(M) 2) Add(N) 3) Set(0)
                        additionally, [(>/<)N (-)M (</>)N -] is similar to divide
                 */
                Op::Jpf => {
                    let loop_len = op.operand - &self.ops[op.operand as usize - 1].operand - 1;
                    if loop_len == 1
                        && self.ops[pos + 1]
                            == (Operation {
                                operator: Op::Upd,
                                operand: -1,
                            })
                    {
                        optimized.push(Operation {
                            operator: Op::Set,
                            operand: 0,
                        });
                        pos += 3;
                    } else if loop_len == 4 {
                        let loop_op: Vec<Op> = self.ops[pos + 1..pos + 5]
                            .iter()
                            .map(|op| op.operator.clone())
                            .collect();
                        if loop_op == vec![Op::Shf, Op::Upd, Op::Shf, Op::Upd]
                            && self.ops[pos + 1].operand == -self.ops[pos + 3].operand
                            && self.ops[pos + 2].operand > 0
                            && self.ops[pos + 4].operand == -1
                        {
                            optimized.extend([
                                Operation {
                                    operator: Op::Mul,
                                    operand: self.ops[pos + 2].operand,
                                },
                                Operation {
                                    operator: Op::Add,
                                    operand: self.ops[pos + 1].operand,
                                },
                                Operation {
                                    operator: Op::Set,
                                    operand: 0,
                                },
                            ]);
                            pos += 6;
                        } else {
                            optimized.push(op.clone());
                            pos += 1;
                        }
                    } else {
                        optimized.push(op.clone());
                        pos += 1;
                    }
                }
                _ => {
                    optimized.push(op.clone());
                    pos += 1;
                }
            }
        }
        optimized
    }
}

#[cfg(test)]
mod tests {
    use crate::bf_str::BfStr;
    use std::io::Write;
    use std::{io, path::PathBuf};

    #[test]
    fn test_interpret() -> io::Result<()> {
        let bf_str = BfStr::from_file(&PathBuf::from("./sample/hello.bf"))?;
        let mut ret = Vec::new();
        bf_str._interpret(io::stdin(), &mut ret);
        assert_eq!(ret, b"Hello World!\n");

        ret.clear();
        let bf_str = BfStr::from_file(&PathBuf::from("./sample/392quine.bf"))?;
        bf_str._interpret(io::stdin(), &mut ret);
        assert_eq!(ret, b"->++>+++>+>+>+++>>>>>>>>>>>>>>>>>>>>+>+>++>+++>++>>+++>+>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>+>+>>+++>>+++>>>>>+++>+>>>>>>>>>++>+++>+++>+>>+++>>>+++>+>++>+++>>>+>+>++>+++>+>+>>+++>>>>>>>+>+>>>+>+>++>+++>+++>+>>+++>>>+++>+>++>+++>++>>+>+>++>+++>+>+>>+++>>>>>+++>+>>>>>++>+++>+++>+>>+++>>>+++>+>+++>+>>+++>>+++>>++[[>>+[>]++>++[<]<-]>+[>]<+<+++[<]<+]>+[>]++++>++[[<++++++++++++++++>-]<+++++++++.<]\x1a");

        ret.clear();
        let data = b"Hello\x04";
        let bf_str = BfStr::from_file(&PathBuf::from("./sample/rot13.bf"))?;
        bf_str._interpret(&data[..], &mut ret);
        assert_eq!(ret, b"U\nr\ny\ny\nb\n\x04\n");

        Ok(())
    }

    use std::process::{Command, Stdio};
    use tempfile::NamedTempFile;

    #[test]
    fn test_cc() -> io::Result<()> {
        let bf_path = [
            (
                PathBuf::from("./sample/hello.bf"),
                String::new(),
                String::from("Hello World!\n"),
            ),
            (PathBuf::from("./sample/392quine.bf"), String::new(), String::from("->++>+++>+>+>+++>>>>>>>>>>>>>>>>>>>>+>+>++>+++>++>>+++>+>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>+>+>>+++>>+++>>>>>+++>+>>>>>>>>>++>+++>+++>+>>+++>>>+++>+>++>+++>>>+>+>++>+++>+>+>>+++>>>>>>>+>+>>>+>+>++>+++>+++>+>>+++>>>+++>+>++>+++>++>>+>+>++>+++>+>+>>+++>>>>>+++>+>>>>>++>+++>+++>+>>+++>>>+++>+>+++>+>>+++>>+++>>++[[>>+[>]++>++[<]<-]>+[>]<+<+++[<]<+]>+[>]++++>++[[<++++++++++++++++>-]<+++++++++.<]\x1a")),
            (
                PathBuf::from("./sample/rot13.bf"),
                String::from("Hello\x04"),
                String::from("U\nr\ny\ny\nb\n\x04\n"),
            ),
            (PathBuf::from("./sample/simplify.bf"), String::new(), String::from("A\nA\nA")),
        ];

        for (path, input, output) in &bf_path {
            let temp_file = NamedTempFile::new()?;
            let bf_str = BfStr::from_file(path)?;
            bf_str.cc(temp_file.path(), false);

            let temp_exec = NamedTempFile::new()?;
            let exit_status = Command::new("gcc")
                .args([
                    "-x",
                    "c",
                    "-o",
                    temp_exec.path().to_str().unwrap(),
                    temp_file.path().to_str().unwrap(),
                ])
                .status()?;
            assert!(exit_status.success());

            let temp_exec = temp_exec.into_temp_path();
            let mut child = Command::new(temp_exec.to_str().unwrap())
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;
            if !input.is_empty() {
                let mut stdin = child.stdin.take().unwrap();
                stdin.write_all(input.as_bytes())?;
                drop(stdin);
            }
            assert_eq!(
                child.wait_with_output().unwrap().stdout,
                output.clone().into_bytes()
            );
        }

        Ok(())
    }
}
