use efflux::prelude::*;

/// The struct which will implement the `Reducer` trait.
pub struct DegreeCentralityReducer;

impl Reducer for DegreeCentralityReducer {
    fn reduce(&mut self, key: &[u8], values: &[&[u8]], ctx: &mut Context) {
        ctx.write(key, values.len().to_string().as_bytes());
    }
}
