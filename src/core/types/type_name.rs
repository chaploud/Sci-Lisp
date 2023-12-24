/* core/types/type_name.rs */

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
    Macro,
    Error,
}

use TypeName::*;
impl fmt::Display for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Nil => "nil".to_string(),
            Bool => "bool".to_string(),
            I64 => "i64".to_string(),
            F64 => "f64".to_string(),
            Symbol => "symbol".to_string(),
            Keyword => "keyword".to_string(),
            Regex => "regex".to_string(),
            String => "string".to_string(),
            List => "list".to_string(),
            Vector => "vector".to_string(),
            Map => "map".to_string(),
            Set => "set".to_string(),
            Function => "function".to_string(),
            Macro => "macro".to_string(),
            Error => "error".to_string(),
        };
        write!(f, "{}", str)
    }
}
