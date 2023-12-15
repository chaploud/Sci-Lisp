use crate::define_protocol;
use crate::persistent_list::PersistentListIter;
use crate::persistent_list::ToPersistentListIter;
use crate::persistent_list_map::PersistentListMapIter;
use crate::persistent_list_map::ToPersistentListMapIter;
use crate::persistent_vector::PersistentVectorIter;
use crate::persistent_vector::ToPersistentVector;
use crate::persistent_vector::ToPersistentVectorIter;
use crate::protocol::Protocol;
use crate::value::ToValue;
use crate::value::Value;
use std::rc::Rc;
// @TODO move to protocols::iterable

define_protocol!(
    Iterable,
    PersistentList,
    PersistentListMap,
    PersistentVector
);

pub enum IterableIter {
    PersistentList(PersistentListIter),
    PersistentVector(PersistentVectorIter),
    PersistentListMap(PersistentListMapIter),
}
impl Iterator for IterableIter {
    type Item = Rc<Value>;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IterableIter::PersistentList(plist_giter) => plist_giter.next(),
            IterableIter::PersistentVector(pvector_iter) => pvector_iter.next(),
            IterableIter::PersistentListMap(plist_map_iter) => {
                let maybe_map_entry = plist_map_iter.next();
                if let Some(map_entry) = maybe_map_entry {
                    // In Clojure: [key val]
                    return Some(
                        vec![map_entry.key.clone(), map_entry.val.clone()]
                            .into_vector()
                            .to_rc_value(),
                    );
                }
                None
            }
        }
    }
}
impl Iterable {
    pub fn iter(&self) -> IterableIter {
        match &*self.value {
            Value::PersistentList(plist) => {
                IterableIter::PersistentList(Rc::new(plist.clone()).iter())
            }
            Value::PersistentVector(pvector) => {
                IterableIter::PersistentVector(Rc::new(pvector.clone()).iter())
            }
            Value::PersistentListMap(pmap) => {
                IterableIter::PersistentListMap(Rc::new(pmap.clone()).iter())
            }
            // We are ok panicking in this case because an invariant on the type is the assumption
            // that we only have an Iterable if we were able to convert
            _ => panic!("Called Iterable iter on non-iterable"),
        }
    }
}
