use dyn_clone::DynClone;

use crate::core::types::error::Result;
use crate::core::value::Value;

pub trait Sliceable: DynClone {
    fn len(&self) -> usize;
    fn at(&self, index: i64) -> Option<Value>;
    fn slice(&self, start: Option<i64>, end: Option<i64>, step: Option<i64>) -> Result<Value>;
}
dyn_clone::clone_trait_object!(Sliceable);

pub trait SliceableMut: DynClone {
    fn at_mut(&mut self, index: i64) -> Option<&mut Value>;
}
dyn_clone::clone_trait_object!(SliceableMut);
