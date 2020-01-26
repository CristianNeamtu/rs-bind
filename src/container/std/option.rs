use std::marker::PhantomData;

use error::BindError;
use traits::{Deserializable, Deserializer, Visitor};

struct OptionVisitor<T: Deserializable> {
    _type: PhantomData<T>
}

impl<T: Deserializable> OptionVisitor<T> {
    pub fn new() -> Self {
        OptionVisitor {
            _type: PhantomData,
        }
    }
}

impl<T: Deserializable> Visitor for OptionVisitor<T> {
    type Value = Option<T>;
    type Error = BindError;

    fn visit_none(self) -> Result<Self::Value, Self::Error> {
        Ok(None)
    }

    fn visit_some<D: Deserializer>(self, deserializer: D) -> Result<Self::Value, Self::Error> {
        T::deserialize(deserializer)
            .map(|t| Some(t))
    }
}

impl<T: Deserializable> Deserializable for Option<T> {
    fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, BindError> {
        let visitor: OptionVisitor<T> = OptionVisitor::<T>::new();
        deserializer.deserialize(visitor)
    }
}
