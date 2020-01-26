#[macro_use]
extern crate criterion;
extern crate json;
extern crate rs_bind;
extern crate serde;
extern crate serde_json;

use criterion::Criterion;

use rs_bind::models::simple::Simple;
use rs_bind::traits::Deserializable;

const JSON_BODY: &str = r#"
    {
        "a": 22,
        "b": "test",
        "c": 12.1
    }
    "#;

fn criterion_benchmark(criterion: &mut Criterion) {
    criterion
        .bench_function("[Simple] json parsing using `json`", |b| b.iter(|| {
            json::parse(JSON_BODY).unwrap();
        }));
    criterion
        .bench_function("[Simple] baseline", |b| b.iter(|| {
            let json = json::parse(JSON_BODY).unwrap();
            Simple {
                a: json["a"].as_i32().unwrap(),
                b: json["b"].as_str().unwrap().to_owned(),
                c: json["c"].as_f64().unwrap()
            };
        }));
    criterion
        .bench_function("[Simple] json parsing + deserialization", |b| b.iter(|| {
            let json = json::parse(JSON_BODY).unwrap();
            Simple::unmarshal(json).unwrap()
        }));
    criterion
        .bench_function("[Menu] Serde parsing + deserialization", |b| b.iter(|| {
            serde_json::from_str::<Simple>(JSON_BODY).unwrap()
        }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
