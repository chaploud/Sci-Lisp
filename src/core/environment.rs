/* core/environment.rs */

use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;

use nohash::BuildNoHashHasher;

use crate::core::builtin::constants::*;
use crate::core::builtin::functions::*;
use crate::core::builtin::r#macros::*;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::symbol::Symbol;
use crate::core::value::Value;

type Lookup = HashMap<Symbol, Value, BuildNoHashHasher<u64>>;

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    pub parent: Option<Rc<RefCell<Environment>>>,
    pub current: Rc<RefCell<Lookup>>,
    pub gensym_id: u64,
}

impl Environment {
    pub fn new_root_environment() -> Rc<RefCell<Self>> {
        let result = Rc::new(RefCell::new(Self {
            parent: None,
            current: Rc::new(RefCell::new(HashMap::default())),
            gensym_id: 0,
        }));

        insert_builtin_macros(&mut result.borrow_mut());
        insert_builtin_functions(&mut result.borrow_mut());
        insert_builtin_constants(&mut result.borrow_mut());

        result
    }

    pub fn new_local_environment(parent: Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            parent: Some(parent.clone()),
            current: Rc::new(RefCell::new(HashMap::default())),
            gensym_id: parent.borrow_mut().gensym_id,
        }))
    }

    pub fn get(&self, key: &Symbol) -> Result<Value> {
        if let Some(value) = self.current.borrow().get(key) {
            return Ok(value.clone());
        }
        if let Some(parent) = self.parent.clone() {
            return parent.borrow().get(key);
        }

        Err(Error::Name(key.to_string()))
    }

    pub fn get_key_value(&self, key: &Symbol) -> Result<(Symbol, Value)> {
        if let Some((k, v)) = self.current.borrow().get_key_value(key) {
            return Ok((k.clone(), v.clone()));
        }
        if let Some(parent) = self.parent.clone() {
            return parent.borrow().get_key_value(key);
        }

        Err(Error::Name(key.to_string()))
    }

    pub fn insert(&mut self, key: &Symbol, value: Value) -> Result<()> {
        match self.current.borrow_mut().entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                if entry.key().meta.mutable {
                    if !key.meta.mutable {
                        // overwrite with const
                        return Err(Error::Const(format!("cannot overwrite '{}' with const", key)));
                    }
                    entry.insert(value);
                } else {
                    return Err(Error::Const(format!("cannot overwrite immutable binding '{}'", key)));
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(value);
            }
        };
        Ok(())
    }

    pub fn set(&mut self, key: &Symbol, value: Value) -> Result<()> {
        match self.current.borrow_mut().entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                if entry.key().meta.mutable {
                    entry.insert(value);
                } else {
                    return Err(Error::Const(format!("cannot overwrite immutable binding '{}'", key)));
                }
            }
            Entry::Vacant(_) => {
                if let Some(parent) = self.parent.clone() {
                    parent.borrow_mut().set(key, value)?;
                } else {
                    return Err(Error::Name(key.to_string()));
                }
            }
        }
        Ok(())
    }
}

fn insert_builtin_functions(env: &mut Environment) {
    let _ = env.insert(&SYMBOL_TYPE, Value::Function(Rc::new(TypeFn)));
    let _ = env.insert(&SYMBOL_PRINT, Value::Function(Rc::new(PrintFn)));
    let _ = env.insert(&SYMBOL_INC, Value::Function(Rc::new(IncFn)));
    let _ = env.insert(&SYMBOL_DEC, Value::Function(Rc::new(DecFn)));
    let _ = env.insert(&SYMBOL_ADD, Value::Function(Rc::new(AddFn)));
    let _ = env.insert(&SYMBOL_SUB, Value::Function(Rc::new(SubFn)));
    let _ = env.insert(&SYMBOL_MUL, Value::Function(Rc::new(MulFn)));
    let _ = env.insert(&SYMBOL_DIV, Value::Function(Rc::new(DivFn)));
    let _ = env.insert(&SYMBOL_FLOORDIV, Value::Function(Rc::new(FloorDivFn)));
    let _ = env.insert(&SYMBOL_REM, Value::Function(Rc::new(RemFn)));
    let _ = env.insert(&SYMBOL_EQUAL, Value::Function(Rc::new(EqualFn)));
    let _ = env.insert(&SYMBOL_NOTEQUAL, Value::Function(Rc::new(NotEqualFn)));
    let _ = env.insert(&SYMBOL_IS, Value::Function(Rc::new(IsFn)));
    let _ = env.insert(&SYMBOL_GE, Value::Function(Rc::new(GeFn)));
    let _ = env.insert(&SYMBOL_GT, Value::Function(Rc::new(GtFn)));
    let _ = env.insert(&SYMBOL_LE, Value::Function(Rc::new(LeFn)));
    let _ = env.insert(&SYMBOL_LT, Value::Function(Rc::new(LtFn)));
    let _ = env.insert(&SYMBOL_XOR, Value::Function(Rc::new(XorFn)));
    let _ = env.insert(&SYMBOL_NOT, Value::Function(Rc::new(NotFn)));
    let _ = env.insert(&SYMBOL_ZEROQ, Value::Function(Rc::new(ZeroQFn)));
    let _ = env.insert(&SYMBOL_NILQ, Value::Function(Rc::new(NilQFn)));
    let _ = env.insert(&SYMBOL_TRUEQ, Value::Function(Rc::new(TrueQFn)));
    let _ = env.insert(&SYMBOL_FALSEQ, Value::Function(Rc::new(FalseQFn)));
    let _ = env.insert(&SYMBOL_NUMBERQ, Value::Function(Rc::new(NumberQFn)));
    let _ = env.insert(&SYMBOL_I64Q, Value::Function(Rc::new(I64QFn)));
    let _ = env.insert(&SYMBOL_F64Q, Value::Function(Rc::new(F64QFn)));
    let _ = env.insert(&SYMBOL_EVENQ, Value::Function(Rc::new(EvenQFn)));
    let _ = env.insert(&SYMBOL_ODDQ, Value::Function(Rc::new(OddQFn)));
    let _ = env.insert(&SYMBOL_EMPTYQ, Value::Function(Rc::new(EmptyQFn)));
    let _ = env.insert(&SYMBOL_STRINGQ, Value::Function(Rc::new(StringQFn)));
    let _ = env.insert(&SYMBOL_KEYWORDQ, Value::Function(Rc::new(KeywordQFn)));
    let _ = env.insert(&SYMBOL_SYMBOLQ, Value::Function(Rc::new(SymbolQFn)));
    let _ = env.insert(&SYMBOL_LISTQ, Value::Function(Rc::new(ListQFn)));
    let _ = env.insert(&SYMBOL_VECTORQ, Value::Function(Rc::new(VectorQFn)));
    let _ = env.insert(&SYMBOL_MAPQ, Value::Function(Rc::new(MapQFn)));
    let _ = env.insert(&SYMBOL_SETQ, Value::Function(Rc::new(SetQFn)));
    let _ = env.insert(&SYMBOL_STR, Value::Function(Rc::new(StrFn)));
    let _ = env.insert(&SYMBOL_I64, Value::Function(Rc::new(I64Fn)));
    let _ = env.insert(&SYMBOL_F64, Value::Function(Rc::new(F64Fn)));
    let _ = env.insert(&SYMBOL_LIST, Value::Function(Rc::new(ListFn)));
    let _ = env.insert(&SYMBOL_VECTOR, Value::Function(Rc::new(VectorFn)));
    let _ = env.insert(&SYMBOL_HMAP, Value::Function(Rc::new(HmapFn)));
    let _ = env.insert(&SYMBOL_HSET, Value::Function(Rc::new(HsetFn)));
    let _ = env.insert(&SYMBOL_FIRST, Value::Function(Rc::new(FirstFn)));
    let _ = env.insert(&SYMBOL_REST, Value::Function(Rc::new(RestFn)));
    let _ = env.insert(&SYMBOL_RANGE, Value::Function(Rc::new(RangeFn)));
    let _ = env.insert(&SYMBOL_SQRT, Value::Function(Rc::new(SqrtFn)));
    let _ = env.insert(&SYMBOL_ABS, Value::Function(Rc::new(AbsFn)));
    let _ = env.insert(&SYMBOL_COS, Value::Function(Rc::new(CosFn)));
    let _ = env.insert(&SYMBOL_SIN, Value::Function(Rc::new(SinFn)));
    let _ = env.insert(&SYMBOL_TAN, Value::Function(Rc::new(TanFn)));
    let _ = env.insert(&SYMBOL_ACOS, Value::Function(Rc::new(AcosFn)));
    let _ = env.insert(&SYMBOL_ASIN, Value::Function(Rc::new(AsinFn)));
    let _ = env.insert(&SYMBOL_ATAN, Value::Function(Rc::new(AtanFn)));
    let _ = env.insert(&SYMBOL_LOG, Value::Function(Rc::new(LogFn)));
    let _ = env.insert(&SYMBOL_LN, Value::Function(Rc::new(LnFn)));
    let _ = env.insert(&SYMBOL_LOG10, Value::Function(Rc::new(Log10Fn)));
    let _ = env.insert(&SYMBOL_RAND, Value::Function(Rc::new(RandFn)));
    let _ = env.insert(&SYMBOL_RANDINT, Value::Function(Rc::new(RandIntFn)));
    let _ = env.insert(&SYMBOL_LEN, Value::Function(Rc::new(LenFn)));
    let _ = env.insert(&SYMBOL_JOIN, Value::Function(Rc::new(JoinFn)));
    let _ = env.insert(&SYMBOL_SPLIT, Value::Function(Rc::new(SplitFn)));
    let _ = env.insert(&SYMBOL_REPLACE, Value::Function(Rc::new(ReplaceFn)));
    let _ = env.insert(&SYMBOL_CONCAT, Value::Function(Rc::new(ConcatFn)));
    let _ = env.insert(&SYMBOL_TRIM, Value::Function(Rc::new(TrimFn)));
    let _ = env.insert(&SYMBOL_INQ, Value::Function(Rc::new(InQFn)));
    let _ = env.insert(&SYMBOL_FIND, Value::Function(Rc::new(FindFn)));
    let _ = env.insert(&SYMBOL_COUNT, Value::Function(Rc::new(CountFn)));
    let _ = env.insert(&SYMBOL_UPPER_CASE, Value::Function(Rc::new(UpperCaseFn)));
    let _ = env.insert(&SYMBOL_LOWER_CASE, Value::Function(Rc::new(LowerCaseFn)));
    let _ = env.insert(&SYMBOL_INDEX, Value::Function(Rc::new(IndexFn)));
    let _ = env.insert(&SYMBOL_LOWER_CAMEL, Value::Function(Rc::new(LowerCamelFn)));
    let _ = env.insert(&SYMBOL_UPPER_CAMEL, Value::Function(Rc::new(UpperCamelFn)));
    let _ = env.insert(&SYMBOL_SNAKE_CASE, Value::Function(Rc::new(SnakeCaseFn)));
    let _ = env.insert(&SYMBOL_KEBAB_CASE, Value::Function(Rc::new(KebabCaseFn)));
    let _ = env.insert(&SYMBOL_TITLE_CASE, Value::Function(Rc::new(TitleCaseFn)));
    let _ = env.insert(&SYMBOL_TRAIN_CASE, Value::Function(Rc::new(TrainCaseFn)));
    let _ = env.insert(&SYMBOL_SHOUTY_SNAKE, Value::Function(Rc::new(ShoutySnakeFn)));
    let _ = env.insert(&SYMBOL_SHOUTY_KEBAB, Value::Function(Rc::new(ShoutyKebabFn)));
    let _ = env.insert(&SYMBOL_REPEAT, Value::Function(Rc::new(RepeatFn)));
    let _ = env.insert(&SYMBOL_FIND_ALL, Value::Function(Rc::new(FindAllFn)));
    let _ = env.insert(&SYMBOL_REVERSE, Value::Function(Rc::new(ReverseFn)));
    let _ = env.insert(&SYMBOL_LAST, Value::Function(Rc::new(LastFn)));
    let _ = env.insert(&SYMBOL_SUM, Value::Function(Rc::new(SumFn)));
    let _ = env.insert(&SYMBOL_MEAN, Value::Function(Rc::new(MeanFn)));
    let _ = env.insert(&SYMBOL_MAX, Value::Function(Rc::new(MaxFn)));
    let _ = env.insert(&SYMBOL_MIN, Value::Function(Rc::new(MinFn)));
    let _ = env.insert(&SYMBOL_INDEX_ALL, Value::Function(Rc::new(IndexAllFn)));
    let _ = env.insert(&SYMBOL_SOMEQ, Value::Function(Rc::new(SomeQFn)));
    let _ = env.insert(&SYMBOL_EVERYQ, Value::Function(Rc::new(EveryQFn)));
    let _ = env.insert(&SYMBOL_SORT, Value::Function(Rc::new(SortFn)));
    let _ = env.insert(&SYMBOL_SHUFFLE, Value::Function(Rc::new(ShuffleFn)));
    let _ = env.insert(&SYMBOL_PUSH, Value::Function(Rc::new(PushFn)));
    let _ = env.insert(&SYMBOL_CONS, Value::Function(Rc::new(ConsFn)));
    let _ = env.insert(&SYMBOL_KEYS, Value::Function(Rc::new(KeysFn)));
    let _ = env.insert(&SYMBOL_VALS, Value::Function(Rc::new(ValsFn)));
    let _ = env.insert(&SYMBOL_ITEMS, Value::Function(Rc::new(ItemsFn)));
    let _ = env.insert(&SYMBOL_GET, Value::Function(Rc::new(GetFn)));
    let _ = env.insert(&SYMBOL_UNION, Value::Function(Rc::new(UnionFn)));
    let _ = env.insert(&SYMBOL_INTERSECT, Value::Function(Rc::new(IntersectFn)));
    let _ = env.insert(&SYMBOL_DIFFERENCE, Value::Function(Rc::new(DifferenceFn)));
    let _ = env.insert(&SYMBOL_APPLY, Value::Function(Rc::new(ApplyFn)));
    let _ = env.insert(&SYMBOL_MAP, Value::Function(Rc::new(MapFn)));
    let _ = env.insert(&SYMBOL_FILTER, Value::Function(Rc::new(FilterFn)));
    let _ = env.insert(&SYMBOL_REDUCE, Value::Function(Rc::new(ReduceFn)));
}

fn insert_builtin_macros(env: &mut Environment) {
    let _ = env.insert(&SYMBOL_DEF, Value::Macro(Rc::new(DefMacro)));
    let _ = env.insert(&SYMBOL_CONST, Value::Macro(Rc::new(ConstMacro)));
    let _ = env.insert(&SYMBOL_SETE, Value::Macro(Rc::new(SetEMacro)));
    let _ = env.insert(&SYMBOL_LET, Value::Macro(Rc::new(LetMacro)));
    let _ = env.insert(&SYMBOL_QUOTE, Value::Macro(Rc::new(QuoteMacro)));
    let _ = env.insert(&SYMBOL_SYNTAX_QUOTE, Value::Macro(Rc::new(SyntaxQuoteMacro)));
    let _ = env.insert(&SYMBOL_DO, Value::Macro(Rc::new(DoMacro)));
    let _ = env.insert(&SYMBOL_IF, Value::Macro(Rc::new(IfMacro)));
    let _ = env.insert(&SYMBOL_WHEN, Value::Macro(Rc::new(WhenMacro)));
    let _ = env.insert(&SYMBOL_WHILE, Value::Macro(Rc::new(WhileMacro)));
    let _ = env.insert(&SYMBOL_SWITCH, Value::Macro(Rc::new(SwitchMacro)));
    let _ = env.insert(&SYMBOL_TIME, Value::Macro(Rc::new(TimeMacro)));
    let _ = env.insert(&SYMBOL_DOC, Value::Macro(Rc::new(DocMacro)));
    let _ = env.insert(&SYMBOL_FN, Value::Macro(Rc::new(FnMacro)));
    let _ = env.insert(&SYMBOL_DEFN, Value::Macro(Rc::new(DefnMacro)));
    let _ = env.insert(&SYMBOL_THREAD_FIRST, Value::Macro(Rc::new(ThreadFirstMacro)));
    let _ = env.insert(&SYMBOL_THREAD_LAST, Value::Macro(Rc::new(ThreadLastMacro)));
    let _ = env.insert(&SYMBOL_COND, Value::Macro(Rc::new(CondMacro)));
    let _ = env.insert(&SYMBOL_AND, Value::Macro(Rc::new(AndMacro)));
    let _ = env.insert(&SYMBOL_OR, Value::Macro(Rc::new(OrMacro)));
    let _ = env.insert(&SYMBOL_FOR, Value::Macro(Rc::new(ForMacro)));
    let _ = env.insert(&SYMBOL_GENSYM, Value::Macro(Rc::new(GensymMacro)));
    let _ = env.insert(&SYMBOL_MACRO, Value::Macro(Rc::new(MacroMacro)));
    let _ = env.insert(&SYMBOL_INSERTE, Value::Macro(Rc::new(InsertEMacro)));
    let _ = env.insert(&SYMBOL_REMOVEE, Value::Macro(Rc::new(RemoveEMacro)));
}

fn insert_builtin_constants(env: &mut Environment) {
    let _ = env.insert(&SYMBOL_PI, CONST_PI);
    let _ = env.insert(&SYMBOL_E, CONST_E);
}
