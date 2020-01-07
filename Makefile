.PHONY: pagerank degree-centrality run format

CMD=./tools/hadoop.sh
BINS=./target/release
SMALL=edges-papers.txt
LARGE=edges-patents.txt

pagerank: build
	${CMD} "${BINS}/pagerank-mapper" "${BINS}/pagerank-reducer" pagerank test-matrix.txt

pagerank-preprocess: build
	${CMD} "${BINS}/identity-mapper" "${BINS}/append-reducer" pagerank-preprocess test-list.txt

degree-centrality: build
	${CMD} "${BINS}/degree-centrality-mapper" "${BINS}/degree-centrality-reducer" degree-centrality test-list.txt

build: format
	cargo build --release

format:
	cargo fmt