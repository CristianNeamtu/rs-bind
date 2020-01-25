#[macro_use]
extern crate criterion;
extern crate json;
extern crate rs_bind;

use criterion::{Bencher, Criterion, Fun};
use json::JsonValue;

use rs_bind::models::menu::Menu;
use rs_bind::traits::Deserializable;

const JSON_BODY: &str = r#"
    {
        "restaurant": "Fast-Fast-Food",
        "items": [{
                "name": "Burger",
                "price": 22,
                "vegetarian": false,
                "ingredients": [
                    "meat", "garlic", "onion", "hamburger buns", "mayonnaise", "ketchup"
                ]
            },
            {
                "name": "Pineapple Pizza",
                "price": 32,
                "vegetarian": false
            }
        ]
    }
    "#;

fn bench_seq_fib(b: &mut Bencher, json: &JsonValue) {
    b.iter(|| {
        Menu::deserialize(json).unwrap()
    });
}

fn criterion_benchmark(criterion: &mut Criterion) {
    let json = json::parse(JSON_BODY).unwrap();
    let function = Fun::new("Deserialization", bench_seq_fib);
    criterion
        .bench_functions("Json", vec![function], json);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);