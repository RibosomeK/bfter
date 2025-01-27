mod bf_str;
use bf_str::BfStr;
use clap::Parser;
use std::{fs::File, io::Read, path::PathBuf};

#[derive(Parser)]
struct Cli {
    /// The path of BrainFuck source file
    #[arg(default_value = "./sample/hello.bf")]
    file: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let mut file = File::open(&args.file).unwrap();
    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    println!("Source Code: \n{}", source.as_str());
    let bf_str = BfStr::from(source.as_str());
    println!("Execution result: ");
    bf_str.interpret();
    print!("\n");
}
