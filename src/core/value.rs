/* core/value.rs */

use std::fmt;
use std::hash::Hash;

use pest::iterators::Pair;

use crate::core::parse::Rule;
use crate::core::types::symbol::Symbol;
use crate::core::types::keyword::Keyword;
use crate::core::types::list::List;
use crate::core::types::vector::Vector;
use crate::core::types::map::Map;
use crate::core::types::set::Set;
use crate::core::types::function::Function;
use crate::core::types::r#macro::Macro;
use crate::core::types::type_name::TypeName;

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
    Function(Function<Value>),
    Macro(Macro<Value>)
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Bool(b1), Value::Bool(b2)) => b1 == b2,
            (Value::I64(i1), Value::I64(i2)) => i1 == i2,
            (Value::F64(f1), Value::F64(f2)) => f1 == f2,
            (Value::Symbol(s1), Value::Symbol(s2)) => s1 == s2,
            (Value::Keyword(k1), Value::Keyword(k2)) => k1 == k2,
            (Value::Regex(r1), Value::Regex(r2)) => r1.as_str() == r2.as_str(),
            (Value::String(s1), Value::String(s2)) => s1 == s2,
            (Value::List(l1), Value::List(l2)) => l1 == l2,
            (Value::Vector(v1), Value::Vector(v2)) => v1 == v2,
            (Value::Map(h1), Value::Map(h2)) => h1 == h2,
            (Value::Set(s1), Value::Set(s2)) => s1 == s2,
            (Value::Function(f1), Value::Function(f2)) => f1 == f2,
            (Value::Macro(m1),)
            _ => false,
        }
    }
}

impl Eq for Value {}
impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Nil => Value::Nil.hash(state),
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
            Value::Function(f) => f.hash(state), // TODO:
            Value::SpecialForm(s) => s.hash(state),
        }
    }
}

impl Value {
    pub fn as_nil() -> Result<Value, String> {
        Ok(Value::Nil)
    }
    pub fn as_bool(pair: Pair<Rule>) -> Result<Value, String> {
        let result = pair.as_str().parse::<bool>();
        match result {
            Ok(value) => Ok(Value::Bool(value)),
            Err(err) => Err(err.to_string()),
        }
    }
    pub fn as_i64(pair: Pair<Rule>) -> Result<Value, String> {
        let mut s = pair.as_str().to_string();
        s.retain(|c| c != '_');
        let result = s.parse::<i64>();
        match result {
            Ok(value) => Ok(Value::I64(value)),
            Err(err) => Err(err.to_string()),
        }
    }
    pub fn as_f64(pair: Pair<Rule>) -> Result<Value, String> {
        let result = pair.as_str().parse::<f64>();
        match result {
            Ok(value) => Ok(Value::F64(value)),
            Err(err) => Err(err.to_string()),
        }
    }
    pub fn as_symbol(pair: Pair<Rule>) -> Result<Value, String> {
        let result = pair.as_str().to_string();
        Ok(Value::Symbol(Symbol { value: result }))
    }
    pub fn as_keyword(pair: Pair<Rule>) -> Result<Value, String> {
        let result = pair.as_str().to_string();
        Ok(Value::Keyword(Keyword {
            value: Symbol { value: result },
        }))
    }
    pub fn as_regex(pair: Pair<Rule>) -> Result<Value, String> {
        let result = pair.into_inner().next().unwrap().as_str();
        let regex = regex::Regex::new(&result);
        match regex {
            Ok(value) => Ok(Value::Regex(value)),
            Err(err) => Err(err.to_string()),
        }
    }
    pub fn as_string(pair: Pair<Rule>) -> Result<Value, String> {
        let result = pair.into_inner().next().unwrap().as_str();
        Ok(Value::String(result.to_string()))
    }
    pub fn as_list(values: Vec<Value>) -> Result<Value, String> {
        let list = List::from(values);
        Ok(Value::List(list))
    }
    pub fn as_vector(values: Vec<Value>) -> Result<Value, String> {
        let vector = Vector::from(values);
        Ok(Value::Vector(vector))
    }
    pub fn as_map(values: Vec<(Value, Value)>) -> Result<Value, String> {
        let map = Map::from(values);
        Ok(Value::Map(map))
    }
    pub fn as_set(values: Vec<Value>) -> Result<Value, String> {
        let set = Set::from(values);
        Ok(Value::Set(set))
    }
    pub fn as_special_form(pair: Pair<Rule>) -> Result<Value, String> {
        let result = pair.as_str().to_string();
        Ok(Value::SpecialForm(SpecialForm::from(&result)))
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
            Function(func) => write!(f, "{}", func), // TODO:
            SpecialForm(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Value {
    pub fn type_name(&self) -> String {
        match self {
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
            Value::SpecialForm(_) => TypeName::SpecialForm.to_string(),
        }
    }
    pub fn call_as_sexpr(&self, args: Vec<Value>) -> Result<Value, String> {
        match self {
            Value::Function(func) => func.call(args),
            Value::SpecialForm(special_form) => special_form.call(args),
            _ => Err(format!("{} is not callable", self)),
        }
    }
}
