/* core/types/macro.rs */

use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use dyn_clone::DynClone;

use crate::core::environment::Environment;
use crate::core::types::error::Result;
use crate::core::value::Value;

pub trait Macro: Debug + DynClone {
    fn call(
        &self,
        args: Vec<Value>,
        environment: &Rc<RefCell<Environment>>,
        ast: &mut Vec<Value>,
        evalfn: fn(&Rc<RefCell<Environment>>, &mut Vec<Value>) -> Result<Value>,
    ) -> Result<Value>;
}
dyn_clone::clone_trait_object!(Macro);
