/* main.rs */
// scilisp  # launch REPL
// scilisp xxx.lisp  # run as script
// scilisp -c xxx.lisp  # compile code
// scilisp -l xxx.lisp  # lint code

use clap::Parser;
use std::path::PathBuf;
mod core;

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

    match action {
        Action::Repl => {
            core::repl::repl();
        }
        Action::Execute(file) => {
            core::repl::execute(file);
        }
        Action::Compile(file) => {
            core::compiler::compile(file);
        }
        Action::Lint(file) => {
            core::linter::lint(file);
        }
    }
}