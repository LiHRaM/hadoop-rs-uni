#!/usr/bin/env bash

./tools/hadoop.sh \
    ./target/release/pagerank-mapper \
    ./target/release/pagerank-reducer \
    pagerank-matrix \
    test-matrix.txt \
    ${1-1}