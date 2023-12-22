/* repl.rs */

use std::path::PathBuf;

use rustyline::{error::ReadlineError, DefaultEditor};

use crate::core::utility::try_read_file;
use crate::core::environment::Environment;
use crate::core::parse::parse;
use crate::core::read::read;
use crate::core::eval::eval;
use crate::core::value::Value;

const HISTORY_FILE: &str = "./.scilisp-history.txt";

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
                    Ok(parsed) => {
                        let mut ast = Vec::<Value>::new();
                        read(&mut ast, parsed)?;
                        let value = eval(&mut environment, &mut ast)?;
                        println!("{}", value);
                        buffer.clear();
                    },
                    Err(err) => {
                        // incomplete "", (), [], {}
                        eprintln!("Error: {:?}", err);
                        buffer.clear(); // TODO: remove this line
                    }
                };

                if let Err(err) = rl.add_history_entry(&line) {
                    eprintln!("Error: {:?}", err);
                    break
                }

                if buffer == "exit" {
                    println!("(Bye!)");
                    break
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                println!("(Bye!)");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D");
                println!("(Bye!)");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    if let Err(err) = rl.save_history(HISTORY_FILE) {
        eprintln!("Error: {:?}", err);
    }
    Ok(())
}

pub fn execute(file: Option<PathBuf>) -> Result<(), String> {
    println!("Executing '{}' ...", file.clone().unwrap().to_string_lossy());

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
