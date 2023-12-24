/* core/types/macro.rs */

use std::fmt;

use crate::core::value::Value;

trait Callable {
    fn call(&self, arg: Value) -> Value;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Macro<F: Fn(Value) -> Value>(F);

impl<F: Fn(Value) -> Value> Callable for SpecialForm<F> {
    fn call(&self, arg: Value) -> Value {
        (self.0)(arg)
    }
}

pub enum SpecialForm<F: Fn(Value) -> Value> {
    Def(F),
    Const(F),
    Let(F),
    Setv(F),
    Fn(F),
    If(F),
    Do(F),
    Switch(F),
    For(F),
    While(F),
    Break(F),
    Continue(F),
    Class(F),
    Struct(F),
    Enum(F),
    Macro(F),
}

fn suffix(s: &str) -> String {
    format!("{} (special form)", s)
}

impl fmt::Display for SpecialForm<Value> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpecialForm::Def => write!(f, "{}", suffix("def")),
            SpecialForm::Const => write!(f, "{}", suffix("const")),
            SpecialForm::Let => write!(f, "{}", suffix("let")),
            SpecialForm::Setv => write!(f, "{}", suffix("set!")),
            SpecialForm::Fn => write!(f, "{}", suffix("fn")),
            SpecialForm::If => write!(f, "{}", suffix("if")),
            SpecialForm::Do => write!(f, "{}", suffix("do")),
            SpecialForm::Switch => write!(f, "{}", suffix("switch")),
            SpecialForm::For => write!(f, "{}", suffix("for")),
            SpecialForm::While => write!(f, "{}", suffix("while")),
            SpecialForm::Break => write!(f, "{}", suffix("break")),
            SpecialForm::Continue => write!(f, "{}", suffix("continue")),
            SpecialForm::Class => write!(f, "{}", suffix("class")),
            SpecialForm::Struct => write!(f, "{}", suffix("struct")),
            SpecialForm::Enum => write!(f, "{}", suffix("enum")),
            SpecialForm::Macro => write!(f, "{}", suffix("macro")),
        }
    }
}

impl SpecialForm<Value> {
    pub fn from(s: &str) -> Self {
        match s {
            "def" => SpecialForm::Def,
            "const" => SpecialForm::Const,
            "let" => SpecialForm::Let,
            "set!" => SpecialForm::Setv,
            "fn" => SpecialForm::Fn,
            "if" => SpecialForm::If,
            "do" => SpecialForm::Do,
            "switch" => SpecialForm::Switch,
            "for" => SpecialForm::For,
            "while" => SpecialForm::While,
            "break" => SpecialForm::Break,
            "continue" => SpecialForm::Continue,
            "class" => SpecialForm::Class,
            "struct" => SpecialForm::Struct,
            "enum" => SpecialForm::Enum,
            "macro" => SpecialForm::Macro,
            _ => unreachable!(),
        }
    }
}
