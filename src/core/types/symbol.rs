/* core/types/symbol.rs */

use std::fmt;

#[derive(Eq, Clone, Debug, Hash)]
pub struct Symbol {
    pub name: std::string::String,
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    mod symbol_tests {
        use crate::core::types::symbol::Symbol;

        #[test]
        fn test_symbol() {
            let s1 = Symbol {
                name: "abc".to_string(),
            };
            let s2 = Symbol {
                name: "abc".to_string(),
            };
            let s3 = Symbol {
                name: "def".to_string(),
            };
            assert_eq!(s1, s2);
            assert_ne!(s1, s3);
        }
    }
}
