use dyn_clone::DynClone;

use crate::core::value::Value;

pub trait Sliceable: DynClone {
    fn len(&self) -> usize;
    fn at(&self, index: i64) -> Option<Value>;
    fn slice(&self, start: i64, end: i64, step: i64) -> Value;
}
dyn_clone::clone_trait_object!(Sliceable);
