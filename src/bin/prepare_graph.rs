use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter},
};

use dijkstra_performance_study::graph::Graph;
use dijkstra_performance_study::pbf::*;
use dijkstra_performance_study::{BenchData, BENCH_FILE};

use rand::{distributions::Uniform, prelude::*};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    /// Path to PBF file to prepare for benchmarking
    pbf_file: String,
    /// Number of source and target nodes to generate
    #[structopt(default_value = "3")]
    node_count: usize,
    /// Seed to "randomly" generate sources and targets
    #[structopt(default_value = "42")]
    seed: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Opts {
        pbf_file,
        seed,
        node_count,
    } = Opts::from_args();
    let pbf = BufReader::new(File::open(&pbf_file)?);

    let (nodes, edges) = load_from_pbf(pbf)?;

    let graph = Graph::new(nodes, edges);

    let node_dist = Uniform::new(0, graph.node_count());
    let mut rng = rand_pcg::Pcg64::seed_from_u64(seed);

    let sources = (0..node_count)
        .map(|_| node_dist.sample(&mut rng))
        .map(Into::into)
        .collect();
    let targets = (0..node_count)
        .map(|_| node_dist.sample(&mut rng))
        .map(Into::into)
        .collect();

    println!("generated sources: {:?}", sources);
    println!("generated targets: {:?}", targets);

    let bench_data = BenchData {
        graph,
        sources,
        targets,
    };

    let resource_file = BufWriter::new(File::create(BENCH_FILE)?);

    bincode::serialize_into(resource_file, &bench_data)?;

    Ok(())
}
