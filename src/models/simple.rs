use container::MapAccess;
use error::BindError;
use traits::{Deserializable, Deserializer, Visitor};

pub struct Simple {
    pub a: i32,
    pub b: String,
    pub c: f64
}

/**

*/
struct SimpleVisitor;

impl Visitor for SimpleVisitor {
    type Value = Simple;
    type Error = BindError;

    fn visit_map<A: MapAccess>(self, mut map: A) -> Result<Self::Value, Self::Error> {
        let mut a: Option<i32> = None;
        let mut b: Option<String> = None;
        let mut c: Option<f64> = None;
        while let Some(field) = map.next_key::<String>()? {
            match field.as_str() {
                "a" => a = map.next_value()?,
                "b" => b = map.next_value()?,
                "c" => c = map.next_value()?,
                field => return Err(BindError::from_string(format!("Unrecognized field {}", field)))
            };
        }
        Ok(Simple {
            a: a.unwrap(),
            b: b.unwrap(),
            c: c.unwrap(),
        })
    }
}

impl Deserializable for Simple {
    fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, BindError> {
        deserializer.deserialize(SimpleVisitor)
    }
}