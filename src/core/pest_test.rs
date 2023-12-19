pub enum Value {
    Nil,
    Bool(bool),
    I64(i64),
    F64(f64),
    Symbol,
    Keyword,
    Regex,
    String(std::string::String),
    List,
    Vector,
    HashMap,
    HashSet,
}
