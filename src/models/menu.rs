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
        let menu: Menu = Menu {
            restaurant: map.get_value("restaurant")?.unwrap(),
            items: map.get_value("items")?.unwrap(),
        };
        Ok(menu)
    }
}

impl Deserializable for Menu {
    fn unmarshal<D: Deserializer>(deserializer: D) -> Result<Self, BindError> {
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
        let menu_item = MenuItem {
            name: map.get_value("name")?.unwrap(),
            price: map.get_value("price")?.unwrap(),
            vegetarian: map.get_value("vegetarian")?.unwrap(),
            ingredients: map.get_value("ingredients")?,
        };
        Ok(menu_item)
    }
}

impl Deserializable for MenuItem {
    fn unmarshal<D: Deserializer>(deserializer: D) -> Result<Self, BindError> {
        deserializer.deserialize(MenuItemVisitor)
    }
}