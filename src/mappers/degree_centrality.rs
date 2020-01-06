use efflux::prelude::*;

/// The struct which will implement the `Mapper` trait.
pub struct DegreeCentralityMapper;

/// An empty implementation of the `Mapper` trait.
impl Mapper for DegreeCentralityMapper {
    fn map(&mut self, _key: usize, value: &[u8], ctx: &mut Context) {
        // skip empty
        if value.is_empty() {
            return;
        }

        // parse into a string using the input bytes
        let value = std::str::from_utf8(value).expect("Invalid UTF-8");

        // trim whitespaces
        let value = value.trim();

        // skip comments
        if value.starts_with("#") {
            return;
        }

        // split on newlines to find pairs
        for pair in value.split("\n") {
            let mut pair = pair.split("\t");
            let to = pair.next().expect("Failed to read to");
            ctx.write_fmt(to, 1); // One indegree
        }
    }
}
