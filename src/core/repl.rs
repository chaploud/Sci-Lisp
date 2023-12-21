/* repl.rs */

use std::path::PathBuf;

use pest::iterators::Pair;
use rustyline::{error::ReadlineError, DefaultEditor};

use crate::core::utility::try_read_file;
use crate::core::environment::Environment;
use crate::core::parse::{parse, Rule};
use crate::core::read::read;
use crate::core::eval::eval;

const HISTORY_FILE: &str = "./.scilisp-history.txt";

fn read_eval_print(environment: &mut Environment, program: Pair<'_, Rule>) -> Result<(), String> {
    let value = read(environment, program)?;
    let result = eval(environment, value)?;
    println!("{}", result);
    Ok(())
}

pub fn repl() -> Result<(), String> {
    println!("Sci-Lisp v{}", env!("CARGO_PKG_VERSION"));

    let mut rl = DefaultEditor::new().unwrap();
    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }

    let mut environment = Environment::new(None, None);

    let mut buffer = String::new();
    loop {
        let readline = rl.readline("λ > ");
        match readline {
            Ok(line) => {
                buffer.push_str(&line);
                buffer.push('\n');

                let parsed = parse(&buffer);
                match parsed {
                    Ok(pair) => {
                        read_eval_print(&mut environment, pair);
                        buffer.clear();
                    },
                    Err(err) => {
                        // incomplete "", (), [], {}
                        if err.to_string().contains("incomplete") {
                            continue
                        }
                    }
                };

                rl.add_history_entry(&buffer)?;

                if buffer == "exit" {
                    break
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    rl.save_history(HISTORY_FILE)?;
    println!("(Bye!)");
    Ok(())
}

pub fn execute(file: Option<PathBuf>) -> Result<(), String> {
    println!("Executing '{}' ...", file.clone().unwrap().to_string_lossy());
    let content = try_read_file(&file)?;
    let parsed = parse(&content)?;
    let mut environment = Environment::new(None, None);
    read_eval_print(&mut environment, parsed);
    Ok(())
}
