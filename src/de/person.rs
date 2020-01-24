use container::MapAccess;
use error::BindError;
use traits::{Deserializable, Deserializer, Visitor};

pub struct Person {
    pub name: String,
    pub age: u16,
    pub happy: bool,
}

impl Person {
    pub fn new(name: String, age: u16, happy: bool) -> Self {
        Person { name, age, happy }
    }
}

#[derive(Debug)]
pub enum PersonFields {
    Name,
    Age,
    Happy,
}

pub struct FieldVisitor;

impl Visitor for FieldVisitor {
    type Value = PersonFields;
    type Error = BindError;

    fn visit_string(self, v: String) -> Result<Self::Value, Self::Error> {
        match v.as_str() {
            "name" => Ok(PersonFields::Name),
            "age" => Ok(PersonFields::Age),
            "happy" => Ok(PersonFields::Happy),
            field => Err(BindError::new(format!("Field {} was not recognized", field)))
        }
    }
}

impl Deserializable for PersonFields {
    fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, BindError> {
        deserializer.deserialize(FieldVisitor)
    }
}

pub struct PersonVisitor;

impl Visitor for PersonVisitor {
    type Value = Person;
    type Error = BindError;

    fn visit_map<A: MapAccess>(self, mut map: A) -> Result<Self::Value, Self::Error> {
        let mut name_optional: Option<String> = None;
        let mut age_optional: Option<u16> = None;
        let mut happy_optional: Option<bool> = None;
        while let Some(key) = map.next_key::<PersonFields>()? {
            match key {
                PersonFields::Age => age_optional = map.next_value()?,
                PersonFields::Name => name_optional = map.next_value()?,
                PersonFields::Happy => happy_optional = map.next_value()?,
            }
        }

        Ok(Person::new(
            name_optional.unwrap(),
            age_optional.unwrap(),
            happy_optional.unwrap()))
    }
}

impl Deserializable for Person {
    fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, BindError> {
        deserializer.deserialize(PersonVisitor)
    }
}
