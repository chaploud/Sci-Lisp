/* core/types/keyword.rs */

use std::fmt;
use std::hash::Hash;

use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::function::Function;
use crate::core::value::Value;

use super::error::{arity_error, key_not_found_error};

#[derive(Hash, PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub struct Keyword {
    pub name: std::string::String,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

// call keyword
impl Function for Keyword {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }
        let result = match args[0] {
            Value::Map(ref m) => match m.get(&Value::Keyword(self.clone())) {
                Some(value) => value.clone(),
                None => return Err(key_not_found_error(Value::Keyword(self.clone()))),
            },
            _ => {
                return Err(Error::Type(format!("Cannot get {} with {}", args[0].type_name(), self)));
            }
        };
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    mod keyword_tests {
        use crate::core::types::keyword::Keyword;

        #[test]
        fn test_keyword() {
            let k1 = Keyword {
                name: ":abc".to_string(),
            };
            let k2 = Keyword {
                name: ":abc".to_string(),
            };
            let k3 = Keyword {
                name: ":def".to_string(),
            };
            assert_eq!(k1, k2);
            assert_ne!(k1, k3);
        }
    }
}
