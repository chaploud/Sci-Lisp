/* core/types/macro.rs */

use std::fmt::Debug;

use dyn_clone::DynClone;

use crate::core::environment::Environment;
use crate::core::types::error::Result;
use crate::core::value::Value;

pub trait Macro: Debug + DynClone {
    fn call(
        &self,
        args: Vec<Value>,
        environment: &mut Environment,
        ast: &mut Vec<Value>,
        evalfn: fn(&mut Environment, &mut Vec<Value>) -> Result<Value>,
    ) -> Result<Value>;
}
dyn_clone::clone_trait_object!(Macro);
