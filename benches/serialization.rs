use criterion::{black_box, Criterion, criterion_group};
use tsid::TSID;

pub fn to_string_benchmark(c: &mut Criterion) {
    c.bench_function(
        "to string",
         |b| b.iter(|| {
             let _str = TSID::from(black_box(20)).to_string();
         })
    );
}


criterion_group!(serialization, to_string_benchmark);
