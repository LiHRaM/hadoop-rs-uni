//! `Mapper` implementation for the hadoop-rs project.
extern crate hadoop_rs;
use hadoop_rs::mappers::DegreeCentralityMapper;

fn main() {
    // execute the mapping phase
    efflux::run_mapper(DegreeCentralityMapper);
}
