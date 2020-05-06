use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::{fs::File, io::BufReader};

use dijkstra_performance_study::{dijkstra::Dijkstra, BenchData};

fn dijkstra_benchmark(c: &mut Criterion) {
    let resource_file = BufReader::new(File::open("resources/bench.data").unwrap());

    let BenchData {
        graph,
        sources,
        targets,
    } = bincode::deserialize_from(resource_file).unwrap();

    let mut dijkstra = Dijkstra::new(&graph);
    let pairs: Vec<(i64, i64)> = sources
        .iter()
        .flat_map(|s| targets.iter().map(move |t| (*s, *t)))
        .collect();

    c.bench_with_input(
        BenchmarkId::new("dijkstra", "10 X 10 source target combinations"),
        &pairs,
        |b, pairs| {
            b.iter(|| {
                for p in pairs {
                    dijkstra.dist(p.0, p.1);
                }
            })
        },
    );
}

criterion_group!(benches, dijkstra_benchmark);
criterion_main!(benches);
