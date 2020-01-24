use rusqlite::{Connection, NO_PARAMS, Statement};

use de::person::Person;
use traits::Deserializable;

struct TestDriver {
    conn: Connection
}

impl TestDriver {
    fn new() -> Self {
        let result = Connection::open_in_memory().unwrap();
        TestDriver {
            conn: result
        }
    }

    fn set_up(&self) -> bool {
        self.conn.execute(
            "CREATE TABLE person (
                      id              INTEGER PRIMARY KEY,
                      name            TEXT NOT NULL,
                      age             INTEGER NOT NULL,
                      happy           BOOLEAN NOT NULL)",
            rusqlite::params![],
        ).is_ok()
    }

    fn insert(&self, person: Person) -> bool {
        self.conn.execute(
            "INSERT INTO person (name, age, happy)
                      VALUES (?1, ?2, ?3)",
            rusqlite::params![person.name,  person.age, person.happy],
        ).is_ok()
    }

    fn query(&self) -> Statement {
        self.conn.prepare("SELECT name, age, happy FROM person").unwrap()
    }

    fn tear_own(&self) -> bool {
        let result = self.conn
            .execute("DROP TABLE person", rusqlite::params![]).is_ok();
        result
    }
}

#[test]
fn should_map_person_from_sql() {
    let test = TestDriver::new();
    test.set_up();

    test.insert(Person::new(String::from("Paul"), 22, true));

    let mut statement = test.query();

    let iter = statement.query_map(NO_PARAMS, |row| {
        Ok(Person::deserialize(row).unwrap())
    });

    let option = iter.unwrap().next().unwrap();

    assert_eq!(option.is_ok(), true);

    let person: Person = option.ok().unwrap();

    assert_eq!(person.happy, true);
    assert_eq!(person.age, 22);
    assert_eq!(person.name, "Paul".to_owned());

    test.tear_own();
}