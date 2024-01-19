/* main.rs */

// scilisp  # launch REPL
// scilisp xxx.lisp  # run as script

use std::path::PathBuf;
use std::process::exit;

use clap::Parser;

mod core;
use crate::core::cli::repl::{execute, repl};

#[derive(Parser)]
#[command(version, about, after_help = "If no arguments are provided, it launches a REPL.")]
struct Args {
    #[arg(help = "Execute <FILE>")]
    file: Option<PathBuf>,
}

#[derive(Debug)]
enum Action {
    Repl,
    Execute(Option<PathBuf>),
}

fn main() {
    let args: Args = Args::parse();

    let action = if args.file.is_some() {
        Action::Execute(args.file)
    } else {
        Action::Repl
    };

    let result = match action {
        Action::Repl => repl(),
        Action::Execute(file) => execute(file),
    };

    if let Err(err) = result {
        eprintln!("{}", err);
        exit(1);
    }
}
