/* core/types/function.rs */

use crate::core::value::Value;

trait Callable {
    fn call(&self, arg: Value) -> Value;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Function<F: Fn(Value) -> Value>(F);

impl<F: Fn(Value) -> Value> Callable for Function<F> {
    fn call(&self, arg: Value) -> Value {
        (self.0)(arg)
    }
}
