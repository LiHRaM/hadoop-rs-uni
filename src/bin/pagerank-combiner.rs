//! `Reducer` implementation for the hadoop-rs project.
extern crate hadoop_rs;
use hadoop_rs::reducers::PageRankCombineReducer;

fn main() {
    // execute the reduction phase
    efflux::run_reducer(PageRankCombineReducer);
}
