/* core/types/keyword.rs */

use std::fmt;
use std::hash::Hash;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Keyword {
    pub value: std::string::String,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    mod keyword_tests {
        use crate::core::types::keyword::Keyword;
        use crate::core::types::symbol::Symbol;

        #[test]
        fn test_keyword() {
            let k1 = Keyword {
                value: Symbol {
                    value: "abc".to_string(),
                },
            };
            let k2 = Keyword {
                value: Symbol {
                    value: "abc".to_string(),
                },
            };
            let k3 = Keyword {
                value: Symbol {
                    value: "def".to_string(),
                },
            };
            assert_eq!(k1, k2);
            assert_ne!(k1, k3);
        }
    }
}
