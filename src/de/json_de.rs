use json::JsonValue;
use json::number::Number;

use container::map_iter::Map;
use traits::{Deserializer, Visitor};
use types::{Numeric, Value};

impl Deserializer for &JsonValue {
    fn deserialize<V: Visitor>(self, visitor: V) -> Result<V::Value, V::Error> {
        Value::from(self).deserialize(visitor)
    }
}

impl Deserializer for JsonValue {
    fn deserialize<V: Visitor>(self, visitor: V) -> Result<V::Value, V::Error> {
        Value::from(self).deserialize(visitor)
    }
}

impl From<JsonValue> for Value {
    fn from(json: JsonValue) -> Self {
        match json {
            JsonValue::Null => Value::Null,
            JsonValue::Boolean(a_bool) => Value::Bool(a_bool),
            JsonValue::Short(short) => Value::String(String::from(short)),
            JsonValue::Number(num) => Value::Number(Numeric::from(num)),
            JsonValue::String(string) => Value::String(string),
            JsonValue::Array(arr) => {
                Value::Array(arr.into_iter().map(Value::from).collect())
            }
            JsonValue::Object(obj) => {
                let map = obj.iter()
                    .map(|(string, val)| {
                        (String::from(string), Value::from(val))
                    }).collect::<Map<String, Value>>();
                Value::Object(map)
            }
        }
    }
}

impl From<Number> for Numeric {
    fn from(number: Number) -> Self {
        let (positive, _, exponent) = number.as_parts();
        match (positive, exponent >= 0) {
            (true, true) => Numeric::U64(u64::from(number)),
            (false, true) => Numeric::I64(i64::from(number)),
            (_, false) => Numeric::F64(f64::from(number))
        }
    }
}


impl From<&JsonValue> for Value {
    fn from(json: &JsonValue) -> Self {
        match json {
            JsonValue::Null => Value::Null,
            JsonValue::Boolean(a_bool) => Value::Bool(*a_bool),
            JsonValue::Short(short) => Value::String(String::from(short.to_owned())),
            JsonValue::Number(num) => Value::Number(Numeric::from(num)),
            JsonValue::String(string) => Value::String(string.to_owned()),
            JsonValue::Array(arr) => {
                Value::Array(arr.into_iter().map(Value::from).collect())
            }
            JsonValue::Object(obj) => {
                let map = obj.iter().map(|(string, val)| {
                    (String::from(string), Value::from(val))
                }).collect::<Map<String, Value>>();
                Value::Object(map)
            }
        }
    }
}

impl From<&Number> for Numeric {
    fn from(number: &Number) -> Self {
        let (positive, _, exponent) = number.as_parts();
        match (positive, exponent >= 0) {
            (true, true) => Numeric::U64(u64::from(*number)),
            (false, true) => Numeric::I64(i64::from(*number)),
            (_, false) => Numeric::F64(f64::from(*number))
        }
    }
}