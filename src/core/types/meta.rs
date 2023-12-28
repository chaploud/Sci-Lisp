use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Meta {
    pub doc: Cow<'static, str>,
    pub mutable: bool,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            doc: Cow::Borrowed(""),
            mutable: true,
        }
    }
}
