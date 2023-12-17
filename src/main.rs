use std::path::{PathBuf, Path};
use clap::Parser;
mod core;

// scilisp  # launch REPL
// scilisp xxx.lisp  # run as script
// scilisp -c xxx.lisp  # compile code
// scilisp -l xxx.lisp  # lint code

#[derive(Parser)]
#[command(version, about, after_help = "If no arguments are provided, it launches a REPL.")]
struct Args {
    #[arg(help = "Execute <FILE>")]
    file: Option<PathBuf>,

    #[arg(short, long, help = "Compile <FILE>", value_name = "FILE")]
    compile: Option<PathBuf>,

    #[arg(short, long, help = "Lint <FILE>", value_name = "FILE", conflicts_with = "compile")]
    lint: Option<PathBuf>,
}

#[derive(Debug)]
enum Action {
    Repl,
    Execute(Option<PathBuf>),
    Compile(Option<PathBuf>),
    Lint(Option<PathBuf>),
}

fn file_not_exist_msg(path: Option<PathBuf>) -> String {
    let path_string = match path {
        Some(p) => format!("{}", p.to_string_lossy().into_owned()),
        None => String::from(""),
    };

    let is_existed = Path::new(&path_string).exists();
    if is_existed {
        format!("{}", path_string)
    } else {
        format!("file '{}' does not exist!", path_string)
    }
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
            println!("'Execute' is not implemented. \n{}", file_not_exist_msg(file));
        }
        Action::Compile(file) => {
            println!("'Compile' is not implemented \n{}", file_not_exist_msg(file));
        }
        Action::Lint(file) => {
            println!("'Lint' is not implemented \n{}", file_not_exist_msg(file));
        }
    }
}
