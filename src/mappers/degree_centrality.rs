use efflux::prelude::*;

pub struct DegreeCentralityMapper;

/// This is basically WordCount but for in degrees
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

        let mut pair = value.split("\t");
        let to = pair.next().expect("Failed to read to");
        ctx.write_fmt(to, 1); // Count one in degree
    }
}
