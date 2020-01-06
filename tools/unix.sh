cat README.md | \
    ./target/release/hadoop-rs_mapper | \
    sort -k1,1 | \
    ./target/release/hadoop-rs_reducer \
    > output_unix.txt