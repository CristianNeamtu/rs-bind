use container::MapAccess;
use error::BindError;
use traits::{Deserializable, Deserializer, Visitor};

#[derive(Debug)]
pub struct Player {
    pub id: u64,
    pub name: String,
    pub nickname: Option<String>,
    pub age: u8,
    pub credits: f32,
    pub has_companion: bool,
    pub game_score: i32,
}

impl Player {
    pub fn new(id: u64, name: String, nickname: Option<String>, age: u8, credits: f32, has_companion: bool, game_score: i32) -> Self {
        Player { id, name, nickname, age, credits, has_companion, game_score }
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
            self.nickname == other.nickname &&
            self.age == other.age &&
            self.credits == other.credits &&
            self.has_companion == other.has_companion &&
            self.game_score == other.game_score
    }
}

#[derive(Debug)]
pub enum PlayerFields {
    Id,
    Name,
    Nickname,
    Age,
    Credits,
    HasCompanion,
    GameScore,
}

pub struct FieldVisitor;

impl Visitor for FieldVisitor {
    type Value = PlayerFields;
    type Error = BindError;

    fn visit_string(self, v: String) -> Result<Self::Value, Self::Error> {
        match v.as_str() {
            "id" => Ok(PlayerFields::Id),
            "name" => Ok(PlayerFields::Name),
            "nickname" => Ok(PlayerFields::Nickname),
            "age" => Ok(PlayerFields::Age),
            "credits" => Ok(PlayerFields::Credits),
            "has_companion" => Ok(PlayerFields::HasCompanion),
            "game_score" => Ok(PlayerFields::GameScore),
            field => Err(BindError::from_string(format!("Field {} was not recognized", field)))
        }
    }
}

impl Deserializable for PlayerFields {
    fn unmarshal<D: Deserializer>(deserializer: D) -> Result<Self, BindError> {
        deserializer.deserialize(FieldVisitor)
    }
}

pub struct PlayerVisitor;

impl Visitor for PlayerVisitor {
    type Value = Player;
    type Error = BindError;

    fn visit_map<A: MapAccess>(self, mut map: A) -> Result<Self::Value, Self::Error> {
        let player = Player::new(
            map.get_value("id")?.unwrap(),
            map.get_value("name")?.unwrap(),
            map.get_value("nickname")?,
            map.get_value("age")?.unwrap(),
            map.get_value("credits")?.unwrap(),
            map.get_value("has_companion")?.unwrap(),
            map.get_value("game_score")?.unwrap());
        Ok(player)
    }
}

impl Deserializable for Player {
    fn unmarshal<D: Deserializer>(deserializer: D) -> Result<Self, BindError> {
        deserializer.deserialize(PlayerVisitor)
    }
}
