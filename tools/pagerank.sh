#!/usr/bin/env bash

make pagerank-preprocess
make pagerank-first
for val in {1..20}; do make pagerank-intermediate; done;
make pagerank-sum