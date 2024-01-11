/* core/cli/repl.rs */

use std::borrow::Cow;
use std::env;
use std::path::PathBuf;

use colored::*;
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::validate::MatchingBracketValidator;
use rustyline::{CompletionType, Config, EditMode, Editor};
use rustyline_derive::{Completer, Helper, Hinter, Validator};

use crate::core::environment::Environment;
use crate::core::eval::eval;
use crate::core::parse::parse;
use crate::core::read::read;
use crate::core::types::error::Result;
use crate::core::utility::try_read_file;
use crate::core::value::Value;

const HISTORY_FILE: &str = "./.scilisp-history.txt";

#[derive(Helper, Hinter, Validator, Completer)]
struct RLHelper {
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,

    #[rustyline(Highlighter)]
    highlighter: MatchingBracketHighlighter,
}

impl Highlighter for RLHelper {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        let highlighted = self.highlighter.highlight(line, pos);

        // HACK:
        let colored = highlighted.replace("\x1b[1;34m", "\x1b[35;47m");

        Cow::Owned(colored)
    }

    fn highlight_char(&self, line: &str, pos: usize, forced: bool) -> bool {
        self.highlighter.highlight_char(line, pos, forced)
    }
}

fn say_goodbye() {
    println!("{}", "[Bye!]".purple());
}

pub fn repl() -> Result<()> {
    println!("Sci-Lisp v{}", env!("CARGO_PKG_VERSION"));

    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();

    let helper = RLHelper {
        highlighter: MatchingBracketHighlighter::new(),
        validator: MatchingBracketValidator::new(),
    };

    let mut rl = Editor::with_config(config).unwrap();
    rl.set_helper(Some(helper));

    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }

    let root = Environment::new_root_environment();
    let environment = Environment::new_local_environment(root.clone());
    // TODO: show completion list from environment (with tab key)

    loop {
        let readline = rl.readline("λ > ".bold().purple().to_string().as_str());
        match readline {
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D");
                say_goodbye();
                break;
            }
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
            Ok(line) => {
                rl.add_history_entry(&line)?;

                if line == "exit" {
                    say_goodbye();
                    break;
                }

                if line.trim().is_empty() {
                    continue;
                }

                // read/parse
                let parsed = parse(&line);
                if let Err(err) = parsed {
                    eprintln!("{}", err);
                    continue;
                }

                let mut ast = Vec::<Value>::new();
                let result = read(&mut ast, parsed.unwrap());
                if let Err(err) = result {
                    eprintln!("{}", err);
                    continue;
                }

                let value = eval(&environment, &mut ast);
                if let Err(err) = value {
                    eprintln!("{}", err);
                    continue;
                }

                println!("{:?}", value.unwrap());
            }
        };
    }

    rl.save_history(HISTORY_FILE)?;
    Ok(())
}

pub fn execute(file: Option<PathBuf>) -> Result<()> {
    // Read
    let content = try_read_file(&file)?;
    let parsed = parse(&content)?;
    let mut ast = Vec::<Value>::new();
    read(&mut ast, parsed)?;

    // Eval
    let root = Environment::new_root_environment();
    let environment = Environment::new_local_environment(root.clone());
    eval(&environment, &mut ast)?;

    Ok(())
}
