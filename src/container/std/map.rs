use std::hash::Hash;
use std::marker::PhantomData;

use container::map_iter::Map;
use container::MapAccess;
use error::BindError;
use traits::{Deserializable, Deserializer, Visitor};

struct MapVisitor<K: Deserializable + Eq + Hash, V: Deserializable> {
    __key: PhantomData<K>,
    __val: PhantomData<V>,
}

impl<K: Deserializable + Eq + Hash, V: Deserializable> MapVisitor<K, V> {
    fn new() -> Self {
        MapVisitor {
            __key: PhantomData,
            __val: PhantomData,
        }
    }
}

impl<K: Deserializable + Eq + Hash, V: Deserializable> Visitor for MapVisitor<K, V> {
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

impl<K: Deserializable + Eq + Hash, V: Deserializable> Deserializable for Map<K, V> {
    fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, BindError> {
        deserializer.deserialize(MapVisitor::<K, V>::new())
    }
}