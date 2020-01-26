#[macro_use]
extern crate criterion;
extern crate json;
extern crate rs_bind;

use criterion::Criterion;

use rs_bind::models::menu::Menu;
use rs_bind::traits::Deserializable;

const JSON_BODY: &str = r#"
    {
        "restaurant": "Fast-Fast-Food",
        "items": [{
                "name": "Burger",
                "price": 22.0,
                "vegetarian": false,
                "ingredients": [
                    "meat", "garlic", "onion", "hamburger buns", "mayonnaise", "ketchup"
                ]
            },
            {
                "name": "Pineapple Pizza",
                "price": 32.0,
                "vegetarian": false
            }
        ]
    }
    "#;

fn criterion_benchmark(criterion: &mut Criterion) {
    criterion
        .bench_function("json deserialization -- menu", |b| b.iter(|| {
            let json = json::parse(JSON_BODY).unwrap();
            Menu::unmarshal(json).unwrap()
        }));
    criterion
        .bench_function("json parse", |b| b.iter(|| {
            json::parse(JSON_BODY).unwrap();
        }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);