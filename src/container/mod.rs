use error::BindError;
use traits::Deserializable;

pub mod map_iter;
pub mod seq_iter;
pub mod std;

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
