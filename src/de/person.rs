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
            field => Err(BindError::new(format!("Field {} was not recognized", field)))
        }
    }
}

impl Deserializable for PlayerFields {
    fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, BindError> {
        deserializer.deserialize(FieldVisitor)
    }
}

pub struct PlayerVisitor;

impl Visitor for PlayerVisitor {
    type Value = Player;
    type Error = BindError;

    fn visit_map<A: MapAccess>(self, mut map: A) -> Result<Self::Value, Self::Error> {
        let mut id: Option<u64> = None;
        let mut name: Option<String> = None;
        let mut nickname: Option<String> = None;
        let mut age: Option<u8> = None;
        let mut credits: Option<f32> = None;
        let mut has_companion: Option<bool> = None;
        let mut game_score: Option<i32> = None;

        while let Some(key) = map.next_key::<PlayerFields>()? {
            match key {
                PlayerFields::Id => id = map.next_value()?,
                PlayerFields::Name => name = map.next_value()?,
                PlayerFields::Nickname => nickname = map.next_value()?,
                PlayerFields::Age => age = map.next_value()?,
                PlayerFields::Credits => credits = map.next_value()?,
                PlayerFields::HasCompanion => has_companion = map.next_value()?,
                PlayerFields::GameScore => game_score = map.next_value()?,
            }
        }
        Ok(Player::new(
            id.unwrap(),
            name.unwrap(),
            nickname,
            age.unwrap(),
            credits.unwrap(),
            has_companion.unwrap(),
            game_score.unwrap())
        )
    }
}

impl Deserializable for Player {
    fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, BindError> {
        deserializer.deserialize(PlayerVisitor)
    }
}
