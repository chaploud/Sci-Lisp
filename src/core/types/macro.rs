/* core/types/macro.rs */

use std::cmp::Ordering;
use std::fmt;

use crate::core::environment::Environment;
use crate::core::types::error::Result;
use crate::core::types::symbol::Symbol;
use crate::core::value::Value;

#[derive(Debug)]
pub struct Macro {
    pub name: Symbol,
    pub func: fn(
        Vec<Value>,
        &mut Environment,
        &mut Vec<Value>,
        fn(&mut Environment, &mut Vec<Value>) -> Result<Value>,
    ) -> Result<Value>,
}
impl Macro {
    pub fn call(
        &self,
        args: Vec<Value>,
        environment: &mut Environment,
        ast: &mut Vec<Value>,
        evalfn: fn(&mut Environment, &mut Vec<Value>) -> Result<Value>,
    ) -> Result<Value> {
        (self.func)(args, environment, ast, evalfn)
    }
}

impl Clone for Macro {
    fn clone(&self) -> Self {
        Macro {
            name: self.name.clone(),
            func: self.func.clone(),
        }
    }
}

impl std::hash::Hash for Macro {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.func.hash(state);
    }
}

impl PartialEq for Macro {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialOrd for Macro {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl Eq for Macro {}

impl Ord for Macro {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl fmt::Display for Macro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
