/* core/types/macro.rs */

use std::borrow::Cow;
use std::fmt;

use crate::core::types::error::Result;
use crate::core::types::ifn::IFn;
use crate::core::value::Value;

pub struct Macro {
    pub name: Cow<'static, str>,
    pub func: fn(Vec<Value>) -> Result<Value>,
}

impl IFn for Macro {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        (self.func)(args)
    }
}

impl std::fmt::Debug for Macro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "macro: '{}' ", self.name)
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
    }
}

impl fmt::Display for Macro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "macro: '{}' ", self.name)
    }
}
