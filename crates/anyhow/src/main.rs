use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ClusterMap {
    name: String,
}

fn get_cluster_info(path: &str) -> Result<ClusterMap> {
    let config = std::fs::read_to_string(path)?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(map)
}

fn main() {
    println!("Hello, anyhow");

    let res = get_cluster_info("cluster.json");
    println!("res: {:?}", res);

    let res = get_cluster_info("cluster_not_found.json");
    println!("res: {:?}", res)
}
