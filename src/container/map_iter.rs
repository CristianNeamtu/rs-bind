#[allow(unused_imports)]
use std::collections::{btree_map, BTreeMap};
#[allow(unused_imports)]
use std::collections::{hash_map, HashMap};

use container::MapAccess;
use error::BindError;
use traits::Deserializable;
use types::Value;

#[cfg(not(feature = "sorted"))]
pub type Map<K, V> = HashMap<K, V>;
#[cfg(not(feature = "sorted"))]
pub type Iterator<K, V> = hash_map::IntoIter<K, V>;

#[cfg(feature = "sorted")]
pub type Map<K, V> = BTreeMap<K, V>;
#[cfg(feature = "sorted")]
pub type Iterator<K, V> = btree_map::IntoIter<K, V>;

pub struct MapIter {
    iterator: Iterator<String, Value>,
    value: Option<Value>,
}

impl MapIter {
    pub fn new(map: Map<String, Value>) -> Self {
        MapIter {
            iterator: map.into_iter(),
            value: None,
        }
    }

    fn map_to_result<T, E>(optional_result: Option<Result<T, E>>) -> Result<Option<T>, E> {
        match optional_result {
            Some(Ok(t)) => {
                Ok(Some(t))
            }
            Some(Err(error)) => {
                Err(error)
            }
            None => {
                Ok(None)
            }
        }
    }
}

impl MapAccess for MapIter {
    fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, BindError> where
        K: Deserializable, V: Deserializable {
        let result = self.iterator.next()
            .map(|(key, value)| {
                let kr = K::deserialize(key);
                let vr = V::deserialize(value);
                kr.and_then(|k| {
                    vr.and_then(|v| Ok((k, v)))
                })
            });
        MapIter::map_to_result(result)
    }

    fn next_key<K: Deserializable>(&mut self) -> Result<Option<K>, BindError> {
        let result = self.iterator.next()
            .map(|(key, value)| {
                self.value = Some(value);
                key
            })
            .map(|key| K::deserialize(key));
        MapIter::map_to_result(result)
    }

    fn next_value<V: Deserializable>(&mut self) -> Result<Option<V>, BindError> {
        let result: Result<Option<V>, BindError> = match self.value.take() {
            Some(value) => {
                V::deserialize(value).map(Option::Some)
            }
            None => Err(BindError::new("Called on nothing".to_string()))
        };
        result
    }
}
