use efflux::prelude::*;

/// The struct which implements the `Mapper` trait using PageRank.
/// Should only be called once.
pub struct PageRankMapper;

/// A PageRank mapper.
/// Emit the initial PageRank for each node.
///
/// Input: <Node1:ID>\t<Node2:ID><Node3>...etc
impl Mapper for PageRankMapper {
    fn map(&mut self, _key: usize, value: &[u8], ctx: &mut Context) {
        // Convert the values into a readable format
        let value = std::str::from_utf8(value).expect("Invalid UTF-8 in data");

        // Separate the ego from the out nodes
        let split = value.split("\t").collect::<Vec<&str>>();
        let key = split[0];
        let nodes = split[1].split(",").collect::<Vec<&str>>();

        // Calculate the initial page rank and set values for the out nodes
        let page_rank = 1.0 / nodes.len() as f64;
        let page_rank = format!("Δ{:.16}", page_rank);
        for node in &nodes {
            ctx.write_fmt(node, &page_rank);
        }

        // Pass the list of out nodes for the ego
        ctx.write_fmt(key, nodes.join(","));
    }
}
