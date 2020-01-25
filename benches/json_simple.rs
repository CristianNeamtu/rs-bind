#[macro_use]
extern crate criterion;
extern crate json;
extern crate rs_bind;

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
        .bench_function("json deserialization -- simple", |b| b.iter(|| {
            let json = json::parse(JSON_BODY).unwrap();
            Simple::deserialize(json).unwrap()
        }));
    criterion
        .bench_function("json parse", |b| b.iter(|| {
            json::parse(JSON_BODY).unwrap();
        }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);