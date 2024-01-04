/* core/value.rs */

use std::cmp::Ordering;
use std::fmt;
use std::hash::Hash;
use std::ops::{Add, Div, Mul, Rem, Sub};

use pest::iterators::Pair;

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

#[derive(Clone, Debug)]
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
            (Function(f1), Function(f2)) => f1.name == f2.name && f1.func == f2.func,
            (Macro(m1), Macro(m2)) => m1.name == m2.name && m1.func == m2.func,
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
        let result = pair.as_str().to_string().into();
        Ok(Value::Symbol(Symbol {
            name: result,
            meta: Default::default(),
        }))
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
}

impl Value {
    // Only nil and false are falsey
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            _ => true,
        }
    }
}

impl Value {
    #[allow(dead_code)]
    pub fn to_i64(&self) -> Result<Value> {
        match self {
            Value::I64(i) => Ok(Value::I64(*i)),
            Value::F64(f) => Ok(Value::I64(*f as i64)),
            Value::String(s) => {
                let result = s.parse::<i64>();
                match result {
                    Ok(value) => Ok(Value::I64(value)),
                    Err(err) => Err(Error::ParseInt(err)),
                }
            }
            _ => Err(Error::Type(format!(
                "Cannot convert {} to i64",
                self.type_name()?
            ))),
        }
    }

    pub fn to_f64(&self) -> Result<Value> {
        match self {
            Value::I64(i) => Ok(Value::F64(*i as f64)),
            Value::F64(f) => Ok(Value::F64(*f)),
            Value::String(s) => {
                let result = s.parse::<f64>();
                match result {
                    Ok(value) => Ok(Value::F64(value)),
                    Err(err) => Err(Error::ParseFloat(err)),
                }
            }
            _ => Err(Error::Type(format!(
                "Cannot convert {} to f64",
                self.type_name()?
            ))),
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        match (self, other) {
            (Value::I64(i1), Value::I64(i2)) => Value::I64(i1 + i2),
            (Value::I64(i1), Value::F64(f2)) => Value::F64(i1 as f64 + f2),
            (Value::F64(f1), Value::I64(i2)) => Value::F64(f1 + i2 as f64),
            (Value::F64(f1), Value::F64(f2)) => Value::F64(f1 + f2),
            (s, o) => panic!("Cannot add {} and {}", s, o),
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        match (self, other) {
            (Value::I64(i1), Value::I64(i2)) => Value::I64(i1 - i2),
            (Value::I64(i1), Value::F64(f2)) => Value::F64(i1 as f64 - f2),
            (Value::F64(f1), Value::I64(i2)) => Value::F64(f1 - i2 as f64),
            (Value::F64(f1), Value::F64(f2)) => Value::F64(f1 - f2),
            (s, o) => panic!("Cannot subtract {} and {}", s, o),
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        match (self, other) {
            (Value::I64(i1), Value::I64(i2)) => Value::I64(i1 * i2),
            (Value::I64(i1), Value::F64(f2)) => Value::F64(i1 as f64 * f2),
            (Value::F64(f1), Value::I64(i2)) => Value::F64(f1 * i2 as f64),
            (Value::F64(f1), Value::F64(f2)) => Value::F64(f1 * f2),
            (s, o) => panic!("Cannot multiply {} and {}", s, o),
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        match (self, other) {
            (Value::I64(i1), Value::I64(i2)) => Value::F64(i1 as f64 / i2 as f64),
            (Value::I64(i1), Value::F64(f2)) => Value::F64(i1 as f64 / f2 as f64),
            (Value::F64(f1), Value::I64(i2)) => Value::F64(f1 as f64 / i2 as f64),
            (Value::F64(f1), Value::F64(f2)) => Value::F64(f1 as f64 / f2 as f64),

            (s, o) => panic!("Cannot divide {} and {}", s, o),
        }
    }
}

impl Value {
    // implement pythonic divide '//' operator
    pub fn floor_div(self, other: Value) -> Value {
        match (self, other) {
            (Value::I64(i1), Value::I64(i2)) => Value::I64(i1 / i2),
            (Value::I64(i1), Value::F64(f2)) => Value::F64((i1 as f64 / f2).floor()),
            (Value::F64(f1), Value::I64(i2)) => Value::F64((f1 / i2 as f64).floor()),
            (Value::F64(f1), Value::F64(f2)) => Value::F64((f1 / f2).floor()),
            (s, o) => panic!("Cannot floor divide {} and {}", s, o),
        }
    }
}

impl Rem for Value {
    type Output = Value;

    fn rem(self, other: Value) -> Value {
        match (self, other) {
            (Value::I64(i1), Value::I64(i2)) => Value::I64(i1 % i2),
            (Value::I64(i1), Value::F64(f2)) => Value::F64(i1 as f64 % f2),
            (Value::F64(f1), Value::I64(i2)) => Value::F64(f1 % i2 as f64),
            (Value::F64(f1), Value::F64(f2)) => Value::F64(f1 % f2),
            (s, o) => panic!("Cannot modulo {} and {}", s, o),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        match (self, other) {
            (Value::I64(i1), Value::I64(i2)) => i1.partial_cmp(i2),
            (Value::I64(i1), Value::F64(f2)) => (*i1 as f64).partial_cmp(f2),
            (Value::F64(f1), Value::I64(i2)) => f1.partial_cmp(&(*i2 as f64)),
            (Value::F64(f1), Value::F64(f2)) => f1.partial_cmp(f2),
            (Value::String(s1), Value::String(s2)) => s1.partial_cmp(s2),
            (Value::Keyword(k1), Value::Keyword(k2)) => k1.partial_cmp(k2),
            (Value::Symbol(s1), Value::Symbol(s2)) => s1.partial_cmp(s2),
            (Value::Regex(r1), Value::Regex(r2)) => r1.as_str().partial_cmp(r2.as_str()),
            (Value::List(l1), Value::List(l2)) => l1.partial_cmp(l2),
            (Value::Vector(v1), Value::Vector(v2)) => v1.partial_cmp(v2),
            (Value::Map(m1), Value::Map(m2)) => m1.partial_cmp(m2),
            (Value::Set(s1), Value::Set(s2)) => s1.partial_cmp(s2),
            (Value::Function(f1), Value::Function(f2)) => f1.partial_cmp(f2),
            (Value::Macro(m1), Value::Macro(m2)) => m1.partial_cmp(m2),
            (Value::Nil, Value::Nil) => Some(Ordering::Equal),
            (Value::Bool(b1), Value::Bool(b2)) => b1.partial_cmp(b2),
            (s, o) => panic!("Cannot compare {:?} and {:?}", s.type_name(), o.type_name()),
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Value) -> Ordering {
        match (self, other) {
            (Value::I64(i1), Value::I64(i2)) => i1.cmp(i2),
            (Value::I64(i1), Value::F64(f2)) => (*i1 as f64).partial_cmp(f2).unwrap(),
            (Value::F64(f1), Value::I64(i2)) => f1.partial_cmp(&(*i2 as f64)).unwrap(),
            (Value::F64(f1), Value::F64(f2)) => f1.partial_cmp(f2).unwrap(),
            (Value::String(s1), Value::String(s2)) => s1.cmp(s2),
            (Value::Keyword(k1), Value::Keyword(k2)) => k1.cmp(k2),
            (Value::Symbol(s1), Value::Symbol(s2)) => s1.cmp(s2),
            (Value::Regex(r1), Value::Regex(r2)) => r1.as_str().cmp(r2.as_str()),
            (Value::List(l1), Value::List(l2)) => l1.cmp(l2),
            (Value::Vector(v1), Value::Vector(v2)) => v1.cmp(v2),
            (Value::Map(m1), Value::Map(m2)) => m1.cmp(m2),
            (Value::Set(s1), Value::Set(s2)) => s1.cmp(s2),
            (Value::Function(f1), Value::Function(f2)) => f1.cmp(f2),
            (Value::Macro(m1), Value::Macro(m2)) => m1.cmp(m2),
            (Value::Nil, Value::Nil) => Ordering::Equal,
            (Value::Bool(b1), Value::Bool(b2)) => b1.cmp(b2),
            (s, o) => panic!("Cannot compare {:?} and {:?}", s.type_name(), o.type_name()),
        }
    }
}
