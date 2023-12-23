/* special_form.rs */

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpecialForm {
    Def,
    Const,
    Let,
    Setv,
    Fn,
    If,
    Do,
    Switch,
    For,
    While,
    Break,
    Continue,
    Class,
    Struct,
    Enum,
    Macro,
}

fn suffix(s: &str) -> String {
    format!("{} (special form)", s)
}

impl fmt::Display for SpecialForm {
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

impl SpecialForm {
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
