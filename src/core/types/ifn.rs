/* core/types/ifn.rs */

use std::fmt::Debug;
use std::rc::Rc;

use dyn_clone::DynClone;

use crate::core::value::Value;

pub trait IFn: Debug + DynClone {
    fn call(&self, args: Vec<Rc<Value>>) -> Value;
}
dyn_clone::clone_trait_object!(IFn);
