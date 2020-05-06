use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::{fs::File, io::BufReader};

use dijkstra_performance_study::{dijkstra::Dijkstra, BenchData, NodeId, BENCH_FILE};

fn dijkstra_benchmark(c: &mut Criterion) {
    let resource_file = BufReader::new(File::open(BENCH_FILE).unwrap());

    let BenchData {
        graph,
        sources,
        targets,
    } = bincode::deserialize_from(resource_file).unwrap();

    let mut dijkstra = Dijkstra::new(&graph);

    let mut group = c.benchmark_group("dijkstra");

    for s in &sources {
        for t in &targets {
            let parameter = format!("{} -> {}", **s, **t);
            group.sample_size(10).bench_with_input(
                BenchmarkId::new("dijkstra", parameter),
                &(s, t),
                |b, pair| b.iter(|| dijkstra.dist(*pair.0, *pair.1)),
            );
        }
    }
}

criterion_group!(benches, dijkstra_benchmark);
criterion_main!(benches);
