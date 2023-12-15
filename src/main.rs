mod core;

use core::env;
use core::object::Object;
use linefeed::{Interface, ReadResult};
use std::cell::RefCell;
use std::rc::Rc;

const PROMPT: &str = "λ > ";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = Interface::new(PROMPT).unwrap();
    let mut env = Rc::new(RefCell::new(env::Env::new()));

    reader.set_prompt(format!("{}", PROMPT).as_ref()).unwrap();

    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        if input.eq("exit") {
            break;
        }
        let val = core::eval::eval(input.as_ref(), &mut env)?;
        match val {
            Object::Void => {}
            Object::Integer(n) => println!("{}", n),
            Object::Bool(b) => println!("{}", b),
            Object::Symbol(s) => println!("{}", s),
            Object::Lambda(params, body, _) => {
                println!("Lambda(");
                for param in params {
                    println!("{} ", param);
                }
                println!(")");
                for expr in (*body).iter() {
                    println!(" {}", expr);
                }
            }
            _ => println!("{}", val),
        }
    }

    println!("(Bye)");
    Ok(())
}
