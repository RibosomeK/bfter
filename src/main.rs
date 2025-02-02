mod bf_str;
use bf_str::BfStr;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    /// Subcommand, run or compile
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        /// The path of BrainFuck source file
        #[arg(default_value = "./sample/hello.bf")]
        file: PathBuf,
    },
    Compile {
        #[arg(short, long, default_value = "./")]
        out: PathBuf,
        /// Compile with some optimizations
        #[arg(short = 'O', long)]
        optimize: bool,
        /// The path of BrainFuck source file
        #[arg(default_value = "./sample/hello.bf")]
        file: PathBuf,
    },
}

fn main() {
    let args = Cli::parse();
    match &args.command {
        Commands::Run { file } => {
            let bf_str = BfStr::from_file(file).unwrap();
            bf_str.interpret();
        }
        Commands::Compile {
            optimize,
            out,
            file,
        } => {
            let bf_str = BfStr::from_file(file).unwrap();
            if let Some(basename) = file.file_stem() {
                let mut c_path = PathBuf::from(out);
                c_path.push(basename);
                c_path.set_extension("c");
                bf_str.cc(&c_path, *optimize);
                println!("Successfully compiled to {:?}", &c_path);
            }
        }
    }
}
