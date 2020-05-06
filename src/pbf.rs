use osmpbfreader::{groups, primitive_block_from_blob, OsmPbfReader};

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{Read, Seek};

use crate::{Edge, Node};

struct PbfEdge {
    nodes: Vec<i64>,
    oneway: bool,
}

fn is_oneway(way: &osmpbfreader::objects::Way) -> bool {
    let one_way = way.tags.get("oneway").and_then(|s| s.parse().ok());
    match one_way {
        Some(rule) => rule,
        None => match way.tags.get("highway").map(|h| h == "motorway") {
            Some(rule) => rule,
            None => false,
        },
    }
}

fn haversine_distance(source: &Node, target: &Node) -> u32 {
    const EARTH_RADIUS: f64 = 6_371_007.2;
    let theta1 = source.lat.to_radians();
    let theta2 = target.lat.to_radians();
    let delta_theta = (target.lat - source.lat).to_radians();
    let delta_lambda = (target.lng - source.lng).to_radians();
    let a = (delta_theta / 2.0).sin().powi(2)
        + theta1.cos() * theta2.cos() * (delta_lambda / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    (EARTH_RADIUS * c).round() as u32
}

pub fn load_from_pbf(pbf: impl Read + Seek) -> Result<(Vec<Node>, Vec<Edge>), Box<dyn Error>> {
    let mut pbf_reader = OsmPbfReader::new(pbf);

    let mut ways = Vec::new();
    for block in pbf_reader
        .blobs()
        .map(|b| primitive_block_from_blob(&b.unwrap()))
    {
        let block = block.unwrap();
        for group in block.get_primitivegroup().iter() {
            for way in groups::ways(&group, &block) {
                if let Some(street_type) = way.tags.get("highway") {
                    match street_type.as_ref() {
                        "footway" | "bridleway" | "steps" | "path" | "cycleway" | "track"
                        | "proposed" | "construction" | "pedestrian" | "rest_area" | "elevator"
                        | "raceway" | "service" | "unclassified" => continue,
                        _ => (),
                    }
                    let nodes = way.nodes.iter().map(|n| n.0).collect();
                    ways.push(PbfEdge {
                        nodes,
                        oneway: is_oneway(&way),
                    });
                }
            }
        }
    }

    println!("loaded {} ways", ways.len());
    let node_ids: HashSet<i64> = ways.iter().flat_map(|c| c.nodes.iter()).copied().collect();
    println!("trying to load {} nodes", node_ids.len());

    pbf_reader.rewind()?;

    let mut nodes = Vec::with_capacity(node_ids.len());
    for block in pbf_reader
        .blobs()
        .map(|b| primitive_block_from_blob(&b.unwrap()))
    {
        let block = block.unwrap();
        for group in block.get_primitivegroup().iter() {
            for node in groups::dense_nodes(&group, &block) {
                if node_ids.contains(&node.id.0) {
                    nodes.push(Node {
                        id: node.id.0,
                        lat: node.lat(),
                        lng: node.lon(),
                    });
                }
            }
        }
    }
    println!("loaded {} nodes", nodes.len());

    let mut edges = Vec::new();

    let id_to_idx: HashMap<_, _> = nodes.iter().enumerate().map(|(i, n)| (n.id, i)).collect();

    for w in &ways {
        for chunk in w.nodes.windows(2) {
            let from = chunk[0];
            let to = chunk[1];
            let dist = haversine_distance(&nodes[id_to_idx[&from]], &nodes[id_to_idx[&to]]);
            edges.push(Edge {
                id: 0,
                from,
                to,
                dist,
            });
            if !w.oneway {
                edges.push(Edge {
                    id: 0,
                    from: to,
                    to: from,
                    dist,
                });
            }
        }
    }

    println!("created {} edges", edges.len());

    Ok((nodes, edges))
}
