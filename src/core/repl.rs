/* repl.rs */

use crate::core::environment::Environment;
use crate::core::eval::eval;
use crate::core::parse::parse;
use crate::core::read::read;
use crate::core::utility::try_read_file;
use crate::core::value::Value;

use colored::*;
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::validate::MatchingBracketValidator;
use rustyline::{CompletionType, Config, EditMode, Editor};
use rustyline_derive::{Completer, Helper, Hinter, Validator};

use std::borrow::Cow;
use std::path::PathBuf;

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
    println!("{}", "[Bye!]".bold().purple());
}

pub fn repl() -> Result<(), String> {
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

    let mut environment = Environment::new(None, None);
    // TODO: show completion list from environment (with tab key)

    loop {
        let readline = rl.readline("λ > ".bold().purple().to_string().as_str());
        match readline {
            Ok(line) => {
                let parsed = parse(&line);
                match parsed {
                    Ok(parsed) => {
                        let mut ast = Vec::<Value>::new();
                        read(&mut ast, parsed)?;
                        let value = eval(&mut environment, &mut ast)?;
                        println!("{}", value);
                    }
                    Err(err) => {
                        eprintln!("{:?}", err);
                    }
                };

                if let Err(err) = rl.add_history_entry(&line) {
                    eprintln!("Error: {:?}", err);
                    break;
                }

                if line == "exit" {
                    say_goodbye();
                    break;
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                say_goodbye();
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D");
                say_goodbye();
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    if let Err(err) = rl.save_history(HISTORY_FILE) {
        eprintln!("Error: {:?}", err);
    }
    Ok(())
}

pub fn execute(file: Option<PathBuf>) -> Result<(), String> {
    println!(
        "Executing '{}' ...",
        file.clone().unwrap().to_string_lossy()
    );

    // Read
    let content = try_read_file(&file)?;
    let parsed = parse(&content)?;
    let mut ast = Vec::<Value>::new();
    read(&mut ast, parsed)?;

    // Eval
    let mut environment = Environment::new(None, None);
    eval(&mut environment, &mut ast)?;

    Ok(())
}
