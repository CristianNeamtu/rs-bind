use std::hash::Hash;

use container::map_iter::Map;
use error::BindError;
use traits::{Deserializable, Visitor};

pub mod map_iter;
pub mod seq_iter;

pub trait MapAccess: Sized {
    fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, BindError> where
        K: Deserializable, V: Deserializable {
        unimplemented!()
    }

    fn next_key<K: Deserializable>(&mut self) -> Result<Option<K>, BindError> {
        unimplemented!()
    }

    fn next_value<V: Deserializable>(&mut self) -> Result<Option<V>, BindError> {
        unimplemented!()
    }
}

pub trait SeqAccess: Sized {
    fn next_element<T: Deserializable>(&mut self) -> Result<Option<T>, BindError> {
        unimplemented!()
    }
}


impl<K: Deserializable + Eq + Hash, V: Deserializable> Visitor for Map<K, V> {
    type Value = Map<K, V>;
    type Error = BindError;

    fn visit_map<A: MapAccess>(self, mut map: A) -> Result<Self::Value, Self::Error> {
        let mut result: Map<K, V> = Map::new();
        while let Ok(Some((k, v))) = map.next_entry::<K, V>() {
            result.insert(k, v);
        }
        Ok(result)
    }
}

impl<V: Deserializable> Visitor for Vec<V> {
    type Value = Vec<V>;
    type Error = ();

    fn visit_seq<A: SeqAccess>(self, mut seq: A) -> Result<Self::Value, Self::Error> {
        let mut result = Vec::<V>::new();
        while let Ok(Some(item)) = seq.next_element::<V>() {
            result.insert(result.len() - 1, item)
        }
        Ok(result)
    }
}
