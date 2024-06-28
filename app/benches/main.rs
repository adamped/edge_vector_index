use criterion::{criterion_group, criterion_main, Criterion};
use edge_vector_index::{EdgeVectorIndex, Index};

pub fn criterion_benchmark(c: &mut Criterion) {
    let built_index = build_index();
    let comparison_vectors = build_comparison().vectors;

    c.bench_function("main", |b| {
        b.iter(|| edge_vector_index(built_index.clone(), comparison_vectors.clone()))
    });
}

fn build_index() -> EdgeVectorIndex {
    let mut index = EdgeVectorIndex::new();

    let mut indexes: Vec<Index> = Vec::new();
    for _ in 0..100 {
        indexes.push(build_comparison());
    }

    index.init(indexes);

    return index;
}

fn build_comparison() -> Index {
    let mut numbers: Vec<f32> = Vec::new();
    for _ in 0..384 {
        numbers.push(0.1);
    }

    return Index::new(numbers, String::from("Empty"));
}

fn edge_vector_index(index: EdgeVectorIndex, compare: Vec<f32>) {
    index.find_closest_match(&compare);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
