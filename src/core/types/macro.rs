/* core/types/macro.rs */

use std::fmt::Debug;
use std::rc::Rc;

use dyn_clone::DynClone;

use crate::core::environment::Environment;
use crate::core::types::error::Result;
use crate::core::types::symbol::Symbol;
use crate::core::value::Value;

pub trait Macro: Debug + DynClone {
    fn name(&self) -> Symbol;
    fn call(
        &self,
        args: Vec<Rc<Value>>,
        environment: &mut Environment,
        ast: &mut Vec<Rc<Value>>,
        evalfn: fn(&mut Environment, &mut Vec<Rc<Value>>) -> Result<Rc<Value>>,
    ) -> Result<Rc<Value>>;
}
dyn_clone::clone_trait_object!(Macro);
