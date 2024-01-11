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
    Generator,
}

use TypeName::*;
impl fmt::Display for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Nil => "nil",
            Bool => "bool",
            I64 => "i64",
            F64 => "f64",
            Symbol => "symbol",
            Keyword => "keyword",
            Regex => "regex",
            String => "string",
            List => "list",
            Vector => "vector",
            Map => "map",
            Set => "set",
            Function => "function",
            Macro => "macro",
            Generator => "generator",
        };
        write!(f, "{}", str)
    }
}
