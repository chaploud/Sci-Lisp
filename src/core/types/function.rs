/* core/types/function.rs */

use std::fmt::Debug;

use dyn_clone::DynClone;

use crate::core::types::error::Result;
use crate::core::value::Value;

pub trait Function: Debug + DynClone {
    fn call(&self, args: Vec<Value>) -> Result<Value>;
}
dyn_clone::clone_trait_object!(Function);
