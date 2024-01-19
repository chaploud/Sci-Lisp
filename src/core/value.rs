/* core/value.rs */

use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt;
use std::hash::Hash;
use std::ops::{Add, ControlFlow, Div, Mul, Neg, Rem, Sub};
use std::rc::Rc;

use pest::iterators::Pair;

use crate::core::parse::Rule;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::error::{arity_error, index_out_of_range_error, key_not_found_error};
use crate::core::types::function::Function;
use crate::core::types::generator::Generator;
use crate::core::types::keyword::Keyword;
use crate::core::types::list::List;
use crate::core::types::map::Map;
use crate::core::types::r#macro::Macro;
use crate::core::types::r#macro::SplicingMacro;
use crate::core::types::set::Set;
use crate::core::types::slice::Slice;
use crate::core::types::sliceable::Sliceable;
use crate::core::types::symbol::Symbol;
use crate::core::types::type_name::TypeName;
use crate::core::types::vector::Vector;
use crate::core::value::Value::*;

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
    Function(Rc<dyn Function>),
    Macro(Rc<dyn Macro>),
    SplicingMacro(Rc<dyn SplicingMacro>),
    Generator(Rc<RefCell<dyn Generator>>),
    Slice(Rc<Slice>),
    ControlFlow(Rc<ControlFlow<Value, Value>>),
}

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
            (Slice(s1), Slice(s2)) => s1 == s2,
            _ => false,
        }
    }
}

impl Eq for Value {}
impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            I64(i) => i.hash(state),
            Symbol(s) => s.hash(state),
            Keyword(k) => k.hash(state),
            String(s) => s.hash(state),
            _ => panic!("Cannot hash {}", self.type_name()), // OK
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
            String(s) => write!(f, "{}", s),
            List(l) => write!(f, "{}", l),
            Vector(v) => write!(f, "{}", v),
            Map(m) => write!(f, "{}", m),
            Set(s) => write!(f, "{}", s),
            Function(func) => write!(f, "{}", func),
            Macro(mac) => write!(f, "{}", mac),
            Generator(g) => write!(f, "{}", g.borrow()),
            Slice(s) => write!(f, "{}", s),
            ControlFlow(cf) => write!(f, "{:?}", cf),
            _ => panic!("Cannot display {}", self.type_name()), // OK
        }
    }
}

impl fmt::Debug for Value {
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
            Generator(g) => write!(f, "{}", g.borrow()),
            Slice(s) => write!(f, "{}", s),
            ControlFlow(cf) => write!(f, "{:?}", cf),
            _ => panic!("Cannot debug {}", self.type_name()), // OK
        }
    }
}

impl Value {
    pub fn type_name(&self) -> std::string::String {
        let result = match self {
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
            Value::Macro(_) => TypeName::Macro,
            Value::Generator(_) => TypeName::Generator,
            Value::Slice(_) => TypeName::Slice,
            _ => panic!("Cannot get type name of {}", self.type_name()), // OK
        };
        result.to_string()
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
        let result = pair.as_str().to_string().to_owned();
        Ok(Value::Symbol(Symbol {
            name: result.clone().into(),
            meta: Default::default(),
            hash: fxhash::hash(&result),
        }))
    }

    pub fn as_keyword(pair: Pair<Rule>) -> Result<Value> {
        let result = pair.as_str().to_string();
        Ok(Value::Keyword(Keyword { name: result }))
    }

    pub fn as_regex(pair: Pair<Rule>) -> Result<Value> {
        let result = pair.into_inner().next().unwrap().as_str();
        let regex = regex::Regex::new(result);
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
    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Nil)
    }

    pub fn is_i64(&self) -> bool {
        matches!(self, Value::I64(_))
    }
}

impl Value {
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
                "Cannot convert {} to i64.",
                self.type_name()
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
                "Cannot convert {} to f64.",
                self.type_name()
            ))),
        }
    }

    pub fn to_str(&self) -> Result<Value> {
        match self {
            Value::I64(i) => Ok(Value::String(i.to_string())),
            Value::F64(f) => Ok(Value::String(f.to_string())),
            Value::String(s) => Ok(Value::String(s.to_string())),
            Value::Symbol(s) => Ok(Value::String(s.name.to_string())),
            _ => Err(Error::Type(format!(
                "Cannot convert {} to string.",
                self.type_name()
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

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Value {
        match self {
            Value::I64(i) => Value::I64(-i),
            Value::F64(f) => Value::F64(-f),
            s => panic!("Cannot negate {}", s),
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
            (Value::I64(i1), Value::F64(f2)) => Value::F64(i1 as f64 / f2),
            (Value::F64(f1), Value::I64(i2)) => Value::F64(f1 / i2 as f64),
            (Value::F64(f1), Value::F64(f2)) => Value::F64(f1 / f2),

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
        Some(self.cmp(other))
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
            (Value::Nil, Value::Nil) => Ordering::Equal,
            (Value::Bool(b1), Value::Bool(b2)) => b1.cmp(b2),
            (s, o) => panic!("Cannot compare {:?} and {:?}", s.type_name(), o.type_name()),
        }
    }
}

pub struct ValueIter {
    pub value: Value,
    pub current: usize,
    pub generator: Rc<RefCell<dyn Generator>>,
}

impl Iterator for ValueIter {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        match self.value {
            Value::List(ref l) => {
                if self.current < l.value.len() {
                    let result = l.value[self.current].clone();
                    self.current += 1;
                    Some(result)
                } else {
                    None
                }
            }
            Value::Vector(ref v) => {
                if self.current < v.value.len() {
                    let result = v.value[self.current].clone();
                    self.current += 1;
                    Some(result)
                } else {
                    None
                }
            }
            Value::Map(ref m) => {
                if self.current < m.value.len() {
                    let result = m.value[self.current].clone();
                    self.current += 1;
                    Some(result)
                } else {
                    None
                }
            }
            Value::Set(ref s) => {
                if self.current < s.value.len() {
                    let result = s.value[self.current].clone();
                    self.current += 1;
                    Some(result)
                } else {
                    None
                }
            }
            Value::Generator(_) => self.generator.borrow_mut().next(),
            _ => panic!("Cannot iterate over {}", self.value.type_name()),
        }
    }
}

impl IntoIterator for Value {
    type Item = Value;
    type IntoIter = ValueIter;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Value::List(l) => l.into_iter(),
            Value::Vector(v) => v.into_iter(),
            Value::Map(m) => m.into_iter(),
            Value::Set(s) => s.into_iter(),
            Value::Generator(g) => ValueIter {
                value: Value::Generator(g.clone()),
                current: 0,
                generator: g,
            },
            _ => panic!("Cannot iterate over {}", self.type_name()),
        }
    }
}

// call i64
impl Function for i64 {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }
        let result = match args[0] {
            Value::Vector(ref v) => match v.at(*self) {
                Some(value) => value,
                None => return Err(index_out_of_range_error(*self)),
            },
            Value::List(ref l) => match l.at(*self) {
                Some(value) => value,
                None => return Err(index_out_of_range_error(*self)),
            },
            Value::Generator(ref g) => match g.borrow().at(*self) {
                Some(value) => return Ok(value.clone()), // TODO: mutable referenece
                None => return Err(index_out_of_range_error(*self)),
            },
            Value::Map(ref m) => match m.get(&Value::I64(*self)) {
                Some(value) => return Ok(value.clone()),
                None => return Err(key_not_found_error(Value::I64(*self))),
            },
            Value::String(ref s) => match s.at(*self) {
                Some(value) => value,
                None => return Err(index_out_of_range_error(*self)),
            },
            _ => {
                return Err(Error::Type(format!(
                    "Cannot index {} with {}",
                    args[0].type_name(),
                    self
                )));
            }
        };
        Ok(result.clone()) // TODO: mutable reference
    }
}

// call String
impl Function for std::string::String {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }
        let result = match args[0] {
            Value::Map(ref m) => match m.get(&Value::String(self.clone())) {
                Some(value) => value.clone(),
                None => return Err(key_not_found_error(Value::String(self.clone()))),
            },
            _ => {
                return Err(Error::Type(format!(
                    "Cannot get {} with {}",
                    args[0].type_name(),
                    self
                )));
            }
        };
        Ok(result)
    }
}

// sliceable string
impl Sliceable for std::string::String {
    fn len(&self) -> usize {
        self.len()
    }
    fn at(&self, index: i64) -> Option<Value> {
        if index < 0 {
            let index = self.len() as i64 + index;
            if index < 0 {
                return None;
            }
            return Some(Value::String(
                self.chars().nth(index as usize).unwrap().to_string(),
            ));
        }
        if index as usize >= self.len() {
            return None;
        }
        Some(Value::String(
            self.chars().nth(index as usize).unwrap().to_string(),
        ))
    }
    fn slice(&self, start: Option<i64>, end: Option<i64>, step: Option<i64>) -> Result<Value> {
        let mut new_slice = std::string::String::new();

        let step = step.unwrap_or(1);

        if step == 0 {
            return Err(Error::Syntax("step cannot be zero".to_string()));
        }

        if step > 0 {
            let mut start = start.unwrap_or(0);
            let mut end = end.unwrap_or(self.len() as i64);
            if start < 0 {
                start += self.len() as i64;
            }
            if end < 0 {
                end += self.len() as i64;
            }

            start = start.clamp(0, self.len() as i64);
            end = end.clamp(0, self.len() as i64);

            let mut current = start;
            while current < end {
                if let Some(Value::String(s)) = self.at(current) {
                    new_slice.push_str(&s);
                    current += step;
                }
            }
        } else {
            let mut start = start.unwrap_or(-1);
            let mut end = end.unwrap_or(-(self.len() as i64) - 1);

            if start > -1 {
                start -= self.len() as i64;
            }
            if end > -1 {
                end -= self.len() as i64;
            }

            start = start.clamp(-(self.len() as i64) - 1, -1);
            end = end.clamp(-(self.len() as i64) - 1, -1);

            let mut current = start;
            while current > end {
                if let Some(Value::String(s)) = self.at(current) {
                    new_slice.push_str(&s);
                    current += step;
                }
            }
        }
        Ok(Value::String(new_slice))
    }
}
