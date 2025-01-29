use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
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
    pub fn from_file(path: &PathBuf) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut source = String::new();
        file.read_to_string(&mut source)?;
        Ok(BfStr::from(source.as_str()))
    }

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
    "typedef struct {\n",
    "    char* items;\n",
    "    size_t len;\n",
    "    size_t cap;\n",
    "} InputBuf;\n",
    "\n",
    "uint8_t tape_curr(Tape* tape) {\n",
    "    return tape->items[tape->ptr];\n",
    "}\n",
    "\n",
    "void tape_asign(Tape* tape, uint8_t u8) {\n",
    "    tape->items[tape->ptr] = u8;\n",
    "}\n",
    "\n",
    "void tape_shift(Tape* tape, int64_t delta) {\n",
    "    int64_t ret = (int64_t)tape->ptr + delta;\n",
    "    if (ret < 0) assert(0 && \"Tape Underflow!\");\n",
    "    if ((size_t)ret >= tape->len) {\n",
    "        printf(\"WARN: Possible Tape Overflow!\\n\");\n",
    "        printf(\"WARN: Current tape pointer: %zu\\n\", tape->ptr);\n",
    "    }\n",
    "    while ((size_t)ret >= tape->len) {\n",
    "        da_append(tape, 0);\n",
    "    }\n",
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
    "    printf(\"Please input a number (0-255) or a ascii char: \");\n",
    "    InputBuf buf = { 0 };\n",
    "    char* endptr;\n",
    "    uint8_t ret;\n",
    "    while (true) {\n",
    "        char c = fgetc(stdin);\n",
    "        if (c == EOF || c == '\\n') break;\n",
    "        da_append(&buf, c);\n",
    "    }\n",
    "    da_append(&buf, '\\0');\n",
    "    if (buf.items[0] == '\\0') assert(0 && \"Invalid input: Empty String!\");\n",
    "    uint8_t num = strtol(buf.items, &endptr, 10);\n",
    "    if (buf.items == endptr) tape_asign(tape, (uint8_t)buf.items[0]);\n",
    "    else tape_asign(tape, num);\n",
    "    free(buf.items);\n",
    "}\n",
    "\n",
    "void tape_out(Tape* tape, size_t step) {\n",
    "    for (size_t i = 0; i < step; ++i) {\n",
    "        printf(\"%c\", tape_curr(tape));\n",
    "    }\n",
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
    pub fn cc(&self, save_path: &PathBuf) {
        let mut file = File::create(save_path).unwrap();

        let mut cmds: Vec<String> = Vec::new();
        let mut goto_stack: Vec<(usize, &Operation)> = Vec::new();
        for (idx, op) in self.ops.iter().enumerate() {
            match op.operator {
                Op::Inc => cmds.push(format!("    tape_update(&tape, {});\n", op.step)),
                Op::Dec => cmds.push(format!("    tape_update(&tape, -{});\n", op.step)),
                Op::Fwd => cmds.push(format!("    tape_shift(&tape, {});\n", op.step)),
                Op::Bwd => cmds.push(format!("    tape_shift(&tape, -{});\n", op.step)),
                Op::Acp => cmds.push(format!("    tape_in(&tape);\n")),
                Op::Out => cmds.push(format!("    tape_out(&tape, {});\n", op.step)),
                Op::Jpf => {
                    cmds.push(String::new());
                    goto_stack.push((idx, op));
                }
                Op::Jpb => match goto_stack.pop() {
                    Some((goto_idx, goto_op)) => {
                        cmds.push(format!(
                            "    tape_jpb(&tape, jpb{});\n    jpf{}:\n",
                            op.step, goto_op.step
                        ));
                        cmds[goto_idx].push_str(
                            format!(
                                "    tape_jpf(&tape, jpf{});\n    jpb{}:\n",
                                goto_op.step, op.step
                            )
                            .as_str(),
                        );
                    }
                    None => panic!("Unbalanced jump!"),
                },
            }
        }

        let _ = file.write(FILE_HEAD.as_bytes());
        let _ = file.write(MAIN_HEAD.as_bytes());
        for cmd in &cmds {
            let _ = file.write(cmd.as_bytes());
        }
        let _ = file.write(MAIN_TAIL.as_bytes());
    }
}
