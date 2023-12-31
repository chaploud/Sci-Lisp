/* core/types/keyword.rs */

use std::fmt;
use std::hash::Hash;

#[derive(Hash, PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub struct Keyword {
    pub name: std::string::String,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
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
