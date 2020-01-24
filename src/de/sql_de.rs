use std::str;

use rusqlite::Row;
use rusqlite::types::{FromSql, FromSqlError, ValueRef};

use container::map_iter::{Map, MapIter};
use traits::{Deserializer, Visitor};
use types::{Numeric, Value};

impl<'de> Deserializer for &Row<'de> {
    fn deserialize<V: Visitor>(self, visitor: V) -> Result<V::Value, V::Error> {
        let mut map: Map<String, Value> = Map::new();
        match self.column_count() {
            0 => visitor.visit_none(),
            1 => {
                let v = self.get::<usize, Value>(0).unwrap();
                v.deserialize(visitor)
            }
            n => {
                for i in 0..n {
                    let string = self.column_name(i).unwrap().to_owned();
                    let raw_value: Value = self.get(i).unwrap();
                    map.insert(string, raw_value);
                }
                visitor.visit_map(MapIter::new(map))
            }
        }
    }
}

impl FromSql for Value {
    fn column_result(value: ValueRef<'_>) -> Result<Self, FromSqlError> {
        match value {
            ValueRef::Null => Ok(Value::Null),
            ValueRef::Integer(value) => Ok(Value::Number(Numeric::I64(value))),
            ValueRef::Real(value) => Ok(Value::Number(Numeric::F64(value))),
            ValueRef::Blob(blob) |
            ValueRef::Text(blob) => {
                str::from_utf8(blob)
                    .map(String::from)
                    .map(Value::String)
                    .map_err(|e| FromSqlError::Other(Box::new(e)))
            }
        }
    }
}

