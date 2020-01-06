# Hadoop-Rs - Programming Assignment


## Goals
There are five tasks in all.
I have modified the original tasks from the programming assignment to fit my datasets.


### Task 1
Define centrality

### Task 2
I compute the most highly cited *paper* using degree centrality with focus on in-degrees, i.e. most influential paper.

### Task 3
Compute the page-rank of the pages

### Task 4

### Task 5

## Overview
The project uses the Rust programming language to create mappers and reducers for Hadoop.
The Streaming tool provided by the Hadoop project is used to call the mappers and reducers (see more in `tools/hadoop.sh`).
This project uses a small dataset (5.6MB of edges) and a standalone installation of Hadoop.
Two datasets are used, the larger one from http://snap.stanford.edu/data/cit-Patents.html and the smaller one from http://snap.stanford.edu/data/cit-HepPh.html.
Both are citation networks.

## Details
Describe how you solved selected interesting aspects of your task.

## Key Items
Point out key implementation aspects in the code.

## Results
Present your results (e.g. output and performance).