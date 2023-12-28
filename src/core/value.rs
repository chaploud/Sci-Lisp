/* core/value.rs */

use std::fmt;
use std::hash::Hash;

use pest::iterators::Pair;

use crate::core::environment::Environment;
use crate::core::parse::Rule;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::function::Function;
use crate::core::types::keyword::Keyword;
use crate::core::types::list::List;
use crate::core::types::map::Map;
use crate::core::types::r#macro::Macro;
use crate::core::types::set::Set;
use crate::core::types::symbol::Symbol;
use crate::core::types::type_name::TypeName;
use crate::core::types::vector::Vector;

#[derive(Clone)]
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
    Macro(Macro),
}

use crate::core::value::Value::*;

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Nil, Nil) => true,
            (Bool(b1), Bool(b2)) => b1 == b2,
            (I64(i1), I64(i2)) => i1 == i2,
            (F64(f1), F64(f2)) => f1 == f2,
            (Symbol(s1), Symbol(s2)) => s1 == s2,
            (Keyword(k1), Keyword(k2)) => k1 == k2,
            (Regex(r1), Regex(r2)) => r1.as_str() == r2.as_str(),
            (String(s1), String(s2)) => s1 == s2,
            (List(l1), List(l2)) => l1 == l2,
            (Vector(v1), Vector(v2)) => v1 == v2,
            (Map(h1), Map(h2)) => h1 == h2,
            (Set(s1), Set(s2)) => s1 == s2,
            (Function(f1), Function(f2)) => f1.name == f2.name, // TODO: Dubious
            (Macro(m1), Macro(m2)) => m1.name == m2.name,       // TODO: Dubious
            _ => false,
        }
    }
}

impl Eq for Value {}
impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Nil => Value::Nil.hash(state),
            Bool(b) => b.hash(state),
            I64(i) => i.hash(state),
            F64(f) => f.to_bits().hash(state),
            Symbol(s) => s.hash(state),
            Keyword(k) => k.hash(state),
            Regex(r) => r.as_str().hash(state),
            String(s) => s.hash(state),
            List(l) => l.hash(state),
            Vector(v) => v.hash(state),
            Map(h) => h.hash(state),
            Set(s) => s.hash(state),
            Function(f) => f.name.hash(state),
            Macro(m) => m.name.hash(state),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Value::*;
        match self {
            Nil => write!(f, "nil"),
            Bool(b) => write!(f, "{}", b),
            I64(i) => write!(f, "{}", i),
            F64(fl) => write!(f, "{}", fl),
            Symbol(s) => write!(f, "{}", s),
            Keyword(k) => write!(f, "{}", k),
            Regex(r) => write!(f, "#\"{}\"", r),
            String(s) => write!(f, "\"{}\"", s),
            List(l) => write!(f, "{}", l),
            Vector(v) => write!(f, "{}", v),
            Map(m) => write!(f, "{}", m),
            Set(s) => write!(f, "{}", s),
            Function(func) => write!(f, "{}", func),
            Macro(mac) => write!(f, "{}", mac),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Value {
    pub fn type_name(&self) -> Result<Value> {
        let result = match self {
            Value::Nil => TypeName::Nil.to_string(),
            Value::Bool(_) => TypeName::Bool.to_string(),
            Value::I64(_) => TypeName::I64.to_string(),
            Value::F64(_) => TypeName::F64.to_string(),
            Value::Symbol(_) => TypeName::Symbol.to_string(),
            Value::Keyword(_) => TypeName::Keyword.to_string(),
            Value::Regex(_) => TypeName::Regex.to_string(),
            Value::String(_) => TypeName::String.to_string(),
            Value::List(_) => TypeName::List.to_string(),
            Value::Vector(_) => TypeName::Vector.to_string(),
            Value::Map(_) => TypeName::Map.to_string(),
            Value::Set(_) => TypeName::Set.to_string(),
            Value::Function(_) => TypeName::Function.to_string(),
            Value::Macro(_) => TypeName::Macro.to_string(),
        };

        Ok(Value::String(result))
    }
}

impl Value {
    pub fn as_nil() -> Result<Value> {
        Ok(Value::Nil)
    }

    pub fn as_bool(pair: Pair<Rule>) -> Result<Value> {
        let result = pair.as_str().parse::<bool>();
        match result {
            Ok(value) => Ok(Value::Bool(value)),
            Err(err) => Err(Error::ParseBool(err)),
        }
    }

    pub fn as_i64(pair: Pair<Rule>) -> Result<Value> {
        let mut s = pair.as_str().to_string();
        s.retain(|c| c != '_');
        let result = s.parse::<i64>();
        match result {
            Ok(value) => Ok(Value::I64(value)),
            Err(err) => Err(Error::ParseInt(err)),
        }
    }

    pub fn as_f64(pair: Pair<Rule>) -> Result<Value> {
        let result = pair.as_str().parse::<f64>();
        match result {
            Ok(value) => Ok(Value::F64(value)),
            Err(err) => Err(Error::ParseFloat(err)),
        }
    }

    pub fn as_symbol(pair: Pair<Rule>) -> Result<Value> {
        let result = pair.as_str().to_string();
        Ok(Value::Symbol(Symbol { name: result }))
    }

    pub fn as_keyword(pair: Pair<Rule>) -> Result<Value> {
        let result = pair.as_str().to_string();
        Ok(Value::Keyword(Keyword { name: result }))
    }

    pub fn as_regex(pair: Pair<Rule>) -> Result<Value> {
        let result = pair.into_inner().next().unwrap().as_str();
        let regex = regex::Regex::new(&result);
        match regex {
            Ok(value) => Ok(Value::Regex(value)),
            Err(err) => Err(Error::Regex(err)),
        }
    }

    pub fn as_string(pair: Pair<Rule>) -> Result<Value> {
        let result = pair.into_inner().next().unwrap().as_str();
        Ok(Value::String(result.to_string()))
    }

    pub fn as_list(values: Vec<Value>) -> Result<Value> {
        let list = List::from(values);
        Ok(Value::List(list))
    }

    pub fn as_vector(values: Vec<Value>) -> Result<Value> {
        let vector = Vector::from(values);
        Ok(Value::Vector(vector))
    }

    pub fn as_map(values: Vec<(Value, Value)>) -> Result<Value> {
        let map = Map::from(values);
        Ok(Value::Map(map))
    }

    pub fn as_set(values: Vec<Value>) -> Result<Value> {
        let set = Set::from(values);
        Ok(Value::Set(set))
    }
    // TODO: Function, Macro, Error
}

pub trait Collection {
    fn get_value(&self) -> Vec<Value>;
}
