use criterion::{criterion_group, criterion_main, Criterion};

fn bench_encode(c: &mut Criterion) {
    c.bench_function("encode", |b| {
        let mut s = vec![];
        for i in 0..21 {
            let c = match i % 2 {
                0 => 'P',
                1 => 'M',
                _ => unreachable!(),
            };
            s.push(c);
        }
        b.iter(|| {
            vax_number::encode(&s);
        });
    });
}

fn bench_decode(c: &mut Criterion) {
    c.bench_function("decode", |b| {
        let mut s = vec![];
        for i in 0..21 {
            let c = match i % 2 {
                0 => 'P',
                1 => 'M',
                _ => unreachable!(),
            };
            s.push(c);
        }
        let v = vax_number::encode(&s);
        b.iter(|| {
            vax_number::decode(v);
        });
    });
}

criterion_group!(bench, bench_encode, bench_decode,);
criterion_main!(bench);
