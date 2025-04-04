/* core/cli/repl.rs */

use std::borrow::Cow;
use std::cell::RefCell;
use std::env;
use std::path::PathBuf;
use std::rc::Rc;

use colored::*;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::validate::MatchingBracketValidator;
use rustyline::Context;
use rustyline::{CompletionType, Config, EditMode, Editor};
use rustyline_derive::{Helper, Hinter, Validator};

use crate::core::environment::Environment;
use crate::core::eval::eval_ast;
use crate::core::parse::parse;
use crate::core::read::read;
use crate::core::types::error::Result;
use crate::core::utility::try_read_file;
use crate::core::value::Value;

const HISTORY_FILE: &str = "./.scilisp-history.txt";

#[derive(Helper, Hinter, Validator)]
struct RLHelper<'a> {
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,

    #[rustyline(Highlighter)]
    highlighter: MatchingBracketHighlighter,

    environment: &'a Rc<RefCell<Environment>>,
}

impl Completer for RLHelper<'_> {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> std::result::Result<(usize, Vec<Pair>), rustyline::error::ReadlineError> {
        let last_word_start = line[..pos]
            .rfind(|c: char| c.is_whitespace() || c == '(' || c == '[' || c == '{')
            .map_or(0, |i| i + 1);
        let last_word = &line[last_word_start..pos];
        let environment_symbols = self.environment.borrow().get_all_symbols();
        let mut candidates: Vec<Pair> = environment_symbols
            .iter()
            .map(|sym| sym.name.clone())
            .filter(|symbol| symbol.starts_with(last_word))
            .map(|symbol| Pair {
                display: symbol.to_string(),
                replacement: format!("{} ", symbol),
            })
            .collect();

        candidates.sort_by(|a, b| a.display.cmp(&b.display));
        candidates.truncate(20);

        Ok((last_word_start, candidates))
    }
}

impl Highlighter for RLHelper<'_> {
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
    eprintln!("{}", "[Bye!]".purple());
}

pub fn repl() -> Result<()> {
    eprintln!("Sci-Lisp v{}", env!("CARGO_PKG_VERSION"));

    let environment = Environment::new_root_environment();

    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();

    let helper = RLHelper {
        highlighter: MatchingBracketHighlighter::new(),
        validator: MatchingBracketValidator::new(),
        environment: &environment,
    };

    let mut rl = Editor::with_config(config).unwrap();
    rl.set_helper(Some(helper));

    if rl.load_history(HISTORY_FILE).is_err() {
        eprintln!("No previous history.");
    }

    loop {
        let readline = rl.readline("λ > ".bold().purple().to_string().as_str());
        match readline {
            Err(ReadlineError::Interrupted) => {
                eprintln!("Ctrl-C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                eprintln!("Ctrl-D");
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

                // eval
                let value = eval_ast(ast, environment.clone());
                if let Err(err) = value {
                    eprintln!("{}", err);
                    continue;
                }

                // print
                if let Ok(Some(value)) = value {
                    println!("{:?}", value);
                }
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
    let environment = Environment::new_root_environment();
    eval_ast(ast, environment)?;

    Ok(())
}
