/* main.rs */

// scilisp  # launch REPL
// scilisp xxx.lisp  # run as script
// scilisp -c xxx.lisp  # compile code
// scilisp -l xxx.lisp  # lint code

use std::path::PathBuf;
use std::process::exit;

use clap::Parser;

mod core;
use crate::core::ui::compiler::compile;
use crate::core::ui::linter::lint;
use crate::core::ui::repl::{execute, repl};

#[derive(Parser)]
#[command(
    version,
    about,
    after_help = "If no arguments are provided, it launches a REPL."
)]
struct Args {
    #[arg(help = "Execute <FILE>")]
    file: Option<PathBuf>,

    #[arg(short, long, help = "Compile <FILE>", value_name = "FILE")]
    compile: Option<PathBuf>,

    #[arg(
        short,
        long,
        help = "Lint <FILE>",
        value_name = "FILE",
        conflicts_with = "compile"
    )]
    lint: Option<PathBuf>,
}

#[derive(Debug)]
enum Action {
    Repl,
    Execute(Option<PathBuf>),
    Compile(Option<PathBuf>),
    Lint(Option<PathBuf>),
}

fn main() {
    let args: Args = Args::parse();

    let action = if args.compile.is_some() {
        Action::Compile(args.compile)
    } else if args.lint.is_some() {
        Action::Lint(args.lint)
    } else if args.file.is_some() {
        Action::Execute(args.file)
    } else {
        Action::Repl
    };

    let result = match action {
        Action::Repl => repl(),
        Action::Execute(file) => execute(file),
        Action::Compile(file) => compile(file),
        Action::Lint(file) => lint(file),
    };

    if let Err(err) = result {
        eprintln!("{}", err);
        exit(1);
    }
}
