use container::map_iter::Map;
use error::BindError;
use traits::Deserializable;
use types::{Numeric, Value};
use de::person::Person;

#[test]
fn should_read_person_with_matching_types() {
    let mut field_map: Map<String, Value> = Map::new();

    field_map.insert("name".to_owned(), Value::String("harry".to_owned()));
    field_map.insert("age".to_owned(), Value::Number(Numeric::U16(22)));
    field_map.insert("happy".to_owned(), Value::Bool(true));
    let value = Value::Object(field_map);
    let option = Person::deserialize(value);

    assert_eq!(option.is_ok(), true);

    let person: Person = option.ok().unwrap();

    assert_eq!(person.happy, true);
    assert_eq!(person.age, 22);
    assert_eq!(person.name, "harry".to_owned());
}

#[test]
fn should_read_person_from_string() {
    let mut field_map: Map<String, Value> = Map::new();

    field_map.insert("name".to_owned(), Value::String("harry".to_owned()));
    field_map.insert("age".to_owned(), Value::String("22".to_owned()));
    field_map.insert("happy".to_owned(), Value::String("true".to_owned()));

    let value = Value::Object(field_map);
    let option = Person::deserialize(value);

    assert_eq!(option.is_ok(), true);

    let person: Person = option.ok().unwrap();

    assert_eq!(person.happy, true);
    assert_eq!(person.age, 22);
    assert_eq!(person.name, "harry".to_owned());
}


#[test]
fn should_panic_when_reading_person() {
    let mut field_map: Map<String, Value> = Map::new();

    field_map.insert("name".to_owned(), Value::String("harry".to_owned()));
    field_map.insert("age".to_owned(), Value::String("22a".to_owned()));
    field_map.insert("happy".to_owned(), Value::String("true".to_owned()));

    let value = Value::Object(field_map);
    let option = Person::deserialize(value);

    assert_eq!(option.is_err(), true);

    let error: BindError = option.err().unwrap();

    assert_eq!(error.get_message(), "Could not parse string to u16");
}
