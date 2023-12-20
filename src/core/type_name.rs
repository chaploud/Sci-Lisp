/* type_name.rs */

use std::fmt;

#[derive(Debug, Clone)]
pub enum TypeName {
    Nil,
    Bool,
    I64,
    F64,
    Symbol,
    Keyword,
    Regex,
    String,
    List,
    Vector,
    Map,
    Set,
    Function,
}

use TypeName::*;
impl fmt::Display for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Nil => "nil".to_string(),
            Bool => "bool".to_string(),
            I64 => "i64".to_string(),
            F64 => "f64".to_string(),
            Symbol => "sym".to_string(),
            Keyword => "key".to_string(),
            Regex => "regex".to_string(),
            String => "str".to_string(),
            List => "list".to_string(),
            Vector => "v[T]".to_string(),
            Map => "m[K,V]".to_string(),
            Set => "s[T]".to_string(),
            Function => "fn".to_string(),
        };
        write!(f, "{}", str)
    }
}
