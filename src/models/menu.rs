use container::MapAccess;
use error::BindError;
use traits::{Deserializable, Deserializer, Visitor};

pub struct Menu {
    pub restaurant: String,
    pub items: Vec<MenuItem>,
}

pub struct MenuItem {
    pub name: String,
    pub price: f32,
    pub vegetarian: bool,
    pub ingredients: Option<Box<[String]>>,
}

/**

*/
struct MenuVisitor;

impl Visitor for MenuVisitor {
    type Value = Menu;
    type Error = BindError;

    fn visit_map<A: MapAccess>(self, mut map: A) -> Result<Self::Value, Self::Error> {
        let mut restaurant: Option<String> = None;
        let mut items: Option<Vec<MenuItem>> = None;
        while let Some(field) = map.next_key::<String>()? {
            match field.as_str() {
                "restaurant" => restaurant = map.next_value()?,
                "items" => items = map.next_value()?,
                field => return Err(BindError::from_string(format!("Unrecognized field {}", field)))
            };
        }
        Ok(Menu {
            restaurant: restaurant.unwrap(),
            items: items.unwrap(),
        })
    }
}

impl Deserializable for Menu {
    fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, BindError> {
        deserializer.deserialize(MenuVisitor)
    }
}

/**

*/
struct MenuItemVisitor;

impl Visitor for MenuItemVisitor {
    type Value = MenuItem;
    type Error = BindError;

    fn visit_map<A: MapAccess>(self, mut map: A) -> Result<Self::Value, Self::Error> {
        let mut name: Option<String> = None;
        let mut price: Option<f32> = None;
        let mut vegetarian: Option<bool> = None;
        let mut ingredients: Option<Box<[String]>> = None;
        while let Some(field) = map.next_key::<String>()? {
            match field.as_str() {
                "name" => name = map.next_value()?,
                "price" => price = map.next_value()?,
                "vegetarian" => vegetarian = map.next_value()?,
                "ingredients" => ingredients = map.next_value()?,
                field => return Err(BindError::from_string(format!("Unrecognized field {}", field)))
            };
        }
        Ok(MenuItem {
            name: name.unwrap(),
            price: price.unwrap(),
            vegetarian: vegetarian.unwrap(),
            ingredients: ingredients,
        })
    }
}

impl Deserializable for MenuItem {
    fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, BindError> {
        deserializer.deserialize(MenuItemVisitor)
    }
}