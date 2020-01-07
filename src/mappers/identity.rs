use efflux::prelude::*;

/// The struct which implements the `Mapper` trait using PageRank.
/// Should only be called once.
pub struct IdentityMapper;

/// A PageRank mapper.
/// Emits a list of out-links for each node.
impl Mapper for IdentityMapper {
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

        let pair = value.split("\t").collect::<Vec<_>>();
        assert!(pair.len() == 2);
        let from = pair[0];
        let to = pair[1];
        ctx.write_fmt(from, to);
    }
}
