/* core/builtin/macros.rs */

use std::borrow::Cow;

use crate::core::environment::Environment;
use crate::core::types::error::Error;
use crate::core::types::r#macro::Macro;
use crate::core::value::Value;

pub const DEF: Macro = Macro {
    name: Cow::Borrowed("def"),
    func: |args: Vec<Value>, environment: &mut Environment| {
        if args.len() > 2 {
            return Err(Error::Syntax("def: too many arguments".to_string()));
        }
        if args.len() < 2 {
            return Err(Error::Syntax("def: too few arguments".to_string()));
        }

        let symbol = match args[0].clone() {
            Value::Symbol(sym) => sym,
            _ => {
                return Err(Error::Syntax(
                    "def: first argument must be a symbol".to_string(),
                ))
            }
        };

        environment.put(symbol.name.clone(), args[1].clone())?;
        Ok(Value::Symbol(symbol))
    },
};

pub const ALL_MACROS: [Value; 1] = [Value::Macro(DEF)];
