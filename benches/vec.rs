use cellvec::{var::Var, vec_cell::VecCell, vec_cell_trait::VecCellTrait, vec_ref_cell::VecRefCell};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn vec(v: &mut Vec<usize>, sum: &mut usize) {
    v.clear();

    for i in 0..v.capacity() {
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

fn vec_cell<V: VecCellTrait<usize>>(v: &V, sum: &mut usize) {
    v.clear();

    for i in 0..v.capacity() {
        v.push(i * 2 + 1);
    }

    for e in v.entries().step_by(3) {
        e.set(e.get() + 5);
    }

    let mut s = 0;

    for e in v.iter() {
        s += e;
    }

    *sum = s;
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let size = 100_000;
    let mut v1 = Vec::with_capacity(size);
    let v2 = VecCell::with_capacity(size);
    let v3 = VecRefCell::with_capacity(size);
    let mut s = [0; 3];

    c.bench_function("vec", |b| b.iter(|| vec(black_box(&mut v1), black_box(&mut s[2]))));
    c.bench_function("vec cell", |b| b.iter(|| vec_cell(black_box(&v2), black_box(&mut s[0]))));
    c.bench_function("vec ref cell", |b| b.iter(|| vec_cell(black_box(&v3), black_box(&mut s[1]))));

    for v in s {
        println!("{v}");
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
