/* core/types/generator.rs */

use std::fmt::{Debug, Display};

use crate::core::value::Value;

pub trait Generator:
    Debug + Display + Iterator<Item = Value> + DoubleEndedIterator<Item = Value>
{
    fn can_reverse(&self) -> bool;
}
