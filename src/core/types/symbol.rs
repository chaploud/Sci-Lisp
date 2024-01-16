/* core/types/symbol.rs */

use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::core::types::meta::Meta;

#[derive(Clone, Debug)]
pub struct Symbol {
    pub name: Cow<'static, str>,
    pub meta: Meta,
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Symbol {}

impl Hash for Symbol {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        fxhash::hash(&self.name);
    }
}

impl nohash::IsEnabled for Symbol {}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Symbol {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
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
