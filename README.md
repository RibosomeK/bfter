# Bfter

a simple [Brain Fuck](https://brainfuck.org/) interpreter in rust. And can also compile to c.

# Usage

```
cargo run -- --help

Usage: bfter <COMMAND>

Commands:
  run
    Usage: bfter run [FILE]
    Arguments:
      [FILE]  The path of BrainFuck source file [default: ./sample/hello.bf]

  compile
    Usage: bfter compile [OPTIONS] [FILE]
    Arguments:
      [FILE]  The path of BrainFuck source file [default: ./sample/hello.bf]

    Options:
      -o, --out <OUT>  [default: ./]
      -O, --optimize   Compile with some optimizations
  
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

# Build

```
cargo build --release
```

# Changelogs

2025-02-02
  - fix parsing error in some cases
  - add `optimize` option to subcommand `compile`

2025-02-1
  - change `,` behavior to official c implement
  - fix spelling mistake: `asign` -> `assign`

2025-01-29
  - change usage with subcommands
  - add `compile` subcommand to compile to c

2025-01-27
- first init