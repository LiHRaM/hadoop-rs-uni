//! `Mapper` implementation for the hadoop-rs project.
extern crate hadoop_rs;
use hadoop_rs::mappers::PageRankMapper;

fn main() {
    // execute the mapping phase
    efflux::run_mapper(PageRankMapper);
}
