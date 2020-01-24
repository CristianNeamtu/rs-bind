use std::vec::IntoIter;
use container::SeqAccess;
use error::BindError;
use traits::Deserializable;
use types::Value;

pub struct SeqIter {
    iterator: IntoIter<Value>
}

impl SeqIter {
    pub fn new(vector: Vec<Value>) -> Self {
        SeqIter {
            iterator: vector.into_iter()
        }
    }

    fn map_to_result<V, E>(optional_result: Option<Result<V, E>>) -> Result<Option<V>, E> {
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


impl SeqAccess for SeqIter {
    fn next_element<V: Deserializable>(&mut self) -> Result<Option<V>, BindError> {
        let optional_result = self.iterator.next()
            .map(|value| V::deserialize(value));
        SeqIter::map_to_result::<V, BindError>(optional_result)
    }
}
