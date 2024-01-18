/* core/types/lambda.rs */

use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use once_cell::sync::Lazy;

use crate::core::environment::Environment;
use crate::core::eval::eval;
use crate::core::types::error::arity_error;
use crate::core::types::error::arity_error_min;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::function::Function;
use crate::core::types::meta::Meta;
use crate::core::types::symbol::Symbol;
use crate::core::value::Value;

pub static SYMBOL_ANPERSAND: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("&"),
    meta: Meta {
        doc: Cow::Borrowed(""),
        mutable: false,
    },
    hash: fxhash::hash("&"),
});

#[derive(Debug, Clone)]
pub struct Lambda {
    pub args: Vec<Symbol>,
    pub body: Vec<Value>,
    pub environment: Rc<RefCell<Environment>>,
}

impl Function for Lambda {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        let local_env = Environment::new_local_environment(self.environment.clone());

        let argc = self.args.len();

        let mut exist_rest = false;

        if argc >= 2 {
            if self.args[argc - 2] == *SYMBOL_ANPERSAND {
                exist_rest = true;
            }
            let rest_symbol = &self.args[argc - 1];
            local_env.borrow_mut().insert(rest_symbol, Value::Nil)?;
        }

        if exist_rest && args.len() < argc - 2 {
            return Err(arity_error_min(argc - 2, args.len()));
        }

        if !exist_rest && args.len() != self.args.len() {
            return Err(arity_error(self.args.len(), args.len()));
        }

        for (i, arg) in args.iter().enumerate() {
            let sym = &self.args[i];
            if *sym == *SYMBOL_ANPERSAND {
                if !exist_rest {
                    return Err(Error::Type(format!("invalid argument: {}", sym)));
                }
                let rest_sym = &self.args[i + 1];
                let rest_args = Value::as_vector(args[i..].to_vec())?;
                local_env.borrow_mut().set(rest_sym, rest_args)?;
                break;
            }
            let _ = local_env.borrow_mut().insert(sym, arg.clone());
        }

        let mut result = Value::Nil;
        for val in &self.body {
            result = eval(val.clone(), local_env.clone())?;
        }

        Ok(result)
    }
}

impl fmt::Display for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<lambda>")
    }
}
