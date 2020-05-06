use dijkstra_performance_study::{dijkstra::Dijkstra, BenchData, BENCH_FILE};
use std::{error::Error, fs::File, io::BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let BenchData {
        graph,
        sources,
        targets,
    } = bincode::deserialize_from(BufReader::new(File::open(BENCH_FILE)?))?;

    let mut d = Dijkstra::new(&graph);

    for s in &sources {
        for t in &targets {
            let dist = d.dist(*s, *t);
            println!("from: {}, to: {}, Distance: {}", **s, **t, dist);
        }
    }

    Ok(())
}
