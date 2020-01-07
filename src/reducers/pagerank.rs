use efflux::prelude::*;

/// The struct which implements the `Reducer` trait for PageRank.
/// Should be called repeatedly to refine the PageRank.
pub struct PageRankReducer;

/// A PageRank reducer.
/// Receives the same kind of output as it emits.
/// The input is a list of ΔPageRank scores from in nodes, plus a list of all the out nodes.
impl Reducer for PageRankReducer {
    fn reduce(&mut self, key: &[u8], values: &[&[u8]], ctx: &mut Context) {
        // Convert from raw bytes into str
        let values = values
            .iter()
            .map(|value| std::str::from_utf8(value).expect("Invalid UTF-8"))
            .collect::<Vec<&str>>();

        // The page rank and out nodes are both included in the aggregate values.
        let mut page_rank = 0.0;
        let mut out_nodes: Vec<&str> = Vec::new();
        for value in &values {
            if value.starts_with("Δ") {
                page_rank += &value[2..].parse::<f64>().unwrap();
            } else {
                out_nodes = value.split(",").collect::<Vec<&str>>();
            }
        }

        // Compute ΔPageRank for each outgoing node and output to the stage
        let page_rank = page_rank / out_nodes.len() as f64;
        for node in &out_nodes {
            let page_rank = format!("Δ{:.16}", page_rank);
            ctx.write_fmt(node, page_rank);
        }

        // Output the total pagerank for this node as well as outgoing nodes
        if out_nodes.len() > 0 {
            ctx.write(key, out_nodes.join(",").as_bytes());
        }
    }
}

pub struct PageRankCombineReducer;

/// A PageRank reducer.
/// Receives the same kind of output as it emits.
/// The input is a list of ΔPageRank scores from in nodes, plus a list of all the out nodes.
impl Reducer for PageRankCombineReducer {
    fn reduce(&mut self, key: &[u8], values: &[&[u8]], ctx: &mut Context) {
        // Convert from raw bytes into str
        let values = values
            .iter()
            .map(|value| std::str::from_utf8(value).expect("Invalid UTF-8"))
            .collect::<Vec<&str>>();

        // The page rank and out nodes are both included in the aggregate values.
        let mut page_rank = 0.0;
        let mut out_nodes = "";
        for value in &values {
            if value.starts_with("Δ") {
                page_rank += &value[2..].parse::<f64>().unwrap();
            } else {
                out_nodes = value;
            }
        }

        let page_rank = format!("Δ{}", page_rank);
        ctx.write(key, page_rank.as_bytes());

        // Output the total pagerank for this node as well as outgoing nodes
        if out_nodes.len() > 0 {
            ctx.write(key, out_nodes.to_string().as_bytes());
        }
    }
}
