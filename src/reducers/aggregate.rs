use efflux::prelude::*;

/// The struct which implements the `Reducer` trait for PageRank.
/// Should only be called once.
pub struct AggregateReducer;

/// A PageRank combiner.
/// Emits the initial PageRank data for each out-link received.
/// Also emits the list of out-links.
impl Reducer for AggregateReducer {
    fn reduce(&mut self, key: &[u8], values: &[&[u8]], ctx: &mut Context) {
        let out = values
            .iter()
            .map(|value| std::str::from_utf8(value).unwrap())
            .collect::<Vec<&str>>()
            .join(",");
        ctx.write(key, out.as_bytes());
    }
}