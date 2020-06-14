use criterion::{criterion_group, criterion_main, Criterion};

use ruby_lexer::lexers::numeric_literal;

// Timings:
//  s 1e-0
// ms 1e-3
// us 1e-6
// ns 1e-9
// ps 1e-12

pub fn numeric_benchmark(c: &mut Criterion) {
    c.bench_function("parse 0", |b| b.iter(|| numeric_literal("0".into())));
    c.bench_function("parse -0d12_345", |b| {
        b.iter(|| numeric_literal("-0d12_345".into()))
    });
    c.bench_function("parse +1825_345e-12", |b| {
        b.iter(|| numeric_literal("+1825_345e-12".into()))
    });
    c.bench_function("parse foobar", |b| {
        b.iter(|| numeric_literal("foobar".into()))
    });
}

criterion_group!(benches, numeric_benchmark);
criterion_main!(benches);
