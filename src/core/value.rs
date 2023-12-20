/* value.rs */

use crate::core::symbol::Symbol;
use crate::core::keyword::Keyword;
use crate::core::list::List;
use crate::core::vector::Vector;
use crate::core::map::Map;
use crate::core::set::Set;
use crate::core::function::Function;
use crate::core::type_name::TypeName;

use std::hash::Hash;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    I64(i64),
    F64(f64),
    Symbol(Symbol),
    Keyword(Keyword),
    Regex(regex::Regex),
    String(std::string::String),
    List(List),
    Vector(Vector),
    Map(Map),
    Set(Set),
    Function(Function),
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match(self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Bool(b1), Value::Bool(b2))  => b1 == b2,
            (Value::I64(i1), Value::I64(i2))  => i1 == i2,
            (Value::F64(f1), Value::F64(f2))  => f1 == f2,
            (Value::Symbol(s1), Value::Symbol(s2))  => s1 == s2,
            (Value::Keyword(k1), Value::Keyword(k2))  => k1 == k2,
            (Value::Regex(r1), Value::Regex(r2))  => r1.as_str() == r2.as_str(),
            (Value::String(s1), Value::String(s2))  => s1 == s2,
            (Value::List(l1), Value::List(l2))  => l1 == l2,
            (Value::Vector(v1), Value::Vector(v2))  => v1 == v2,
            (Value::Map(h1), Value::Map(h2))  => h1 == h2,
            (Value::Set(s1), Value::Set(s2))  => s1 == s2,
            (Value::Function(_), Value::Function(_))  => false, // TODO:
            _ => false,
        }
    }
}

impl Eq for Value {}
impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Nil => 0.hash(state),
            Value::Bool(b) => b.hash(state),
            Value::I64(i) => i.hash(state),
            Value::F64(f) => f.to_bits().hash(state),
            Value::Symbol(s) => s.hash(state),
            Value::Keyword(k) => k.hash(state),
            Value::Regex(r) => r.as_str().hash(state),
            Value::String(s) => s.hash(state),
            Value::List(l) => l.hash(state),
            Value::Vector(v) => v.hash(state),
            Value::Map(h) => h.hash(state),
            Value::Set(s) => s.hash(state),
            Value::Function(_) => 0.hash(state), // TODO:
        }
    }
}

impl Value {
    pub fn type_name(&self) -> TypeName {
        match self {
            Value::Nil => TypeName::Nil,
            Value::Bool(_) => TypeName::Bool,
            Value::I64(_) => TypeName::I64,
            Value::F64(_) => TypeName::F64,
            Value::Symbol(_) => TypeName::Symbol,
            Value::Keyword(_) => TypeName::Keyword,
            Value::Regex(_) => TypeName::Regex,
            Value::String(_) => TypeName::String,
            Value::List(_) => TypeName::List,
            Value::Vector(_) => TypeName::Vector,
            Value::Map(_) => TypeName::Map,
            Value::Set(_) => TypeName::Set,
            Value::Function(_) => TypeName::Function,
        }
    }
}
