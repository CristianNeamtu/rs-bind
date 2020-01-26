use container::MapAccess;
use error::BindError;
use traits::{Deserializable, Deserializer, Visitor};

pub struct Simple {
    pub a: i32,
    pub b: String,
    pub c: f64,
}

/**

*/
struct SimpleVisitor;

impl Visitor for SimpleVisitor {
    type Value = Simple;
    type Error = BindError;

    fn visit_map<A: MapAccess>(self, mut map: A) -> Result<Self::Value, Self::Error> {
        let simple = Simple {
            a: map.get_value("a")?.unwrap(),
            b: map.get_value("b")?.unwrap(),
            c: map.get_value("c")?.unwrap(),
        };
        Ok(simple)
    }
}

impl Deserializable for Simple {
    fn unmarshal<D: Deserializer>(deserializer: D) -> Result<Self, BindError> {
        deserializer.deserialize(SimpleVisitor)
    }
}