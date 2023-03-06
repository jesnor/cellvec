use cellvec::{
    vec_cell::VecCell,
    vec_cell_trait::{VecCellEntry, VecCellTrait},
    vec_ref_cell::VecRefCell,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn vec(size: usize, sum: &mut usize) {
    let mut v = Vec::with_capacity(size);

    for i in 0..size {
        v.push(i * 2 + 1);
    }

    for e in v.iter_mut().step_by(3) {
        *e += 5;
    }

    let mut s = 0;

    for e in v.iter() {
        s += e;
    }

    *sum = s;
}

fn vec_cell<V: VecCellTrait<usize>>(size: usize, sum: &mut usize) {
    let v = V::with_capacity(size);

    for i in 0..size {
        v.push(i * 2 + 1);
    }

    for e in v.entries().step_by(3) {
        e.set(*e + 5);
    }

    let mut s = 0;

    for e in v.iter() {
        s += e;
    }

    *sum = s;
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let count = 1_000_000;
    let mut s = [0; 3];

    c.bench_function("vec cell", |b| b.iter(|| vec_cell::<VecCell<usize>>(black_box(count), black_box(&mut s[0]))));

    c.bench_function("vec ref cell", |b| {
        b.iter(|| vec_cell::<VecRefCell<usize>>(black_box(count), black_box(&mut s[1])))
    });

    c.bench_function("vec", |b| b.iter(|| vec(black_box(count), black_box(&mut s[2]))));

    for v in s {
        println!("{v}");
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
