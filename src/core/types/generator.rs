/* core/types/generator.rs */

use std::fmt::{Debug, Display};

use crate::core::types::sliceable::Sliceable;
use crate::core::value::Value;

pub trait Generator: Debug + Display + Iterator<Item = Value> + DoubleEndedIterator<Item = Value> + Sliceable {}
