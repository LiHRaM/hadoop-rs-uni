#!/usr/bin/env bash

set -e # Exit on failure

# Set HADOOP_HOME if it does not exist
HADOOP_HOME=${HADOOP_HOME:-"/usr/local/hadoop/"}

mapper=$1           # Path to the mapper binary
reducer=$2          # Path to the reducer binary
output="output/$3"  # If 3 is empty, then the output folder is used
input="data/$4"     # Look for files in the data folder
combiner=$5         # Optional combiner

if [ ! -f "$mapper" ]; then
    echo "Mapper missing";
    exit 1;
fi

if [ ! -f "$reducer" ]; then
    echo "Missing reducer";
    exit 1;
fi

echo "Using mapper: $mapper"
echo "Using reducer: $reducer"
echo "Output folder: $output"

if [ ! -f ./tools/streaming.jar ]; then
    echo "Streaming.jar not found. Did you call me from the wrong directory?";
    exit 1;
fi

# if [ ! -f "$input" ]; then
#     echo "File $input not found!";
#     exit 1;
# fi

if [ -d "$output" ]; then
    rm -r "$output";
fi

if [ ! -d "$HADOOP_HOME" ]; then
    echo "This script assumes hadoop is installed in /usr/local/hadoop";
    exit 1;
fi

if [ -d "$combiner" ]; then
    "$HADOOP_HOME/bin/hadoop" jar ./tools/streaming.jar \
        -input $input \
        -output $output \
        -mapper $mapper \
        -reducer $reducer \
        -combiner $combiner >> output/make.log 2>> output/make.log
else
    "$HADOOP_HOME/bin/hadoop" jar ./tools/streaming.jar \
        -input $input \
        -output $output \
        -mapper $mapper \
        -reducer $reducer >> output/make.log 2>> output/make.log
fi