use binary_heap::{BinaryHeap, HeapKind};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::distributions::Standard;
// use rand::seq::{IteratorRandom, SliceRandom};
use rand::{thread_rng, Rng};
// use std::collections::hash_map::RandomState;

pub fn benchmark_insert(c: &mut Criterion) {
    let mut rng = thread_rng();
    let num_items_to_insert: usize = black_box(10000);
    let data: Vec<i32> = (&mut rng)
        .sample_iter(Standard)
        .take(num_items_to_insert)
        .collect();
    c.bench_function("benchmark insert", |b| {
        b.iter(|| {
            // data.iter().for_each(|item| heap.insert(item));
            BinaryHeap::<i32, fnv::FnvBuildHasher>::heapify(&data, HeapKind::Min);
        })
    });
}

pub fn benchmark_extract_object(c: &mut Criterion) {
    let mut rng = thread_rng();
    let num_items_to_insert: usize = black_box(10000);
    let data: Vec<i32> = (&mut rng)
        .sample_iter(Standard)
        .take(num_items_to_insert)
        .collect();
    c.bench_function("benchmark extract object", |b| {
        b.iter(|| {
            let mut heap = BinaryHeap::<i32, fnv::FnvBuildHasher>::heapify(&data, HeapKind::Min);
            for _ in 0..num_items_to_insert {
                heap.extract_object();
            }
        })
    });
}

criterion_group!(benches, benchmark_insert, benchmark_extract_object);
criterion_main!(benches);
