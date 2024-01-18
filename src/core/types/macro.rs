/* core/types/macro.rs */

use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::ops::ControlFlow;
use std::rc::Rc;

use dyn_clone::DynClone;

use crate::core::environment::Environment;
use crate::core::types::error::Result;
use crate::core::value::Value;

pub trait Macro: Debug + Display + DynClone {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value>;
}
dyn_clone::clone_trait_object!(Macro);

pub trait ControlFlowMacro: Debug + Display + DynClone {
    fn call(&self, args: Vec<Value>) -> Result<ControlFlow<Value, Value>>;
}
dyn_clone::clone_trait_object!(ControlFlowMacro);

pub trait SplicingMacro: Debug + Display + DynClone {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Vec<Value>>;
}
dyn_clone::clone_trait_object!(SplicingMacro);
