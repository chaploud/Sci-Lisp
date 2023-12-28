/* core/types/symbol.rs */

use std::hash::{Hash, Hasher};
use std::{borrow::Cow, fmt};

use crate::core::types::meta::Meta;

#[derive(Eq, Clone, Debug, PartialOrd, Ord)]
pub struct Symbol {
    pub name: Cow<'static, str>,
    pub meta: Meta,
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
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
                name: "abc".into(),
                meta: Default::default(),
            };
            let s2 = Symbol {
                name: "abc".into(),
                meta: Default::default(),
            };
            let s3 = Symbol {
                name: "def".into(),
                meta: Default::default(),
            };
            assert_eq!(s1, s2);
            assert_ne!(s1, s3);
        }
    }
}
