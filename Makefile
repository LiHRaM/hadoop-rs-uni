.PHONY: degree-centrality run format

CMD=./tools/hadoop.sh
BINS=./target/release
SMALL=edges-papers.txt
LARGE=edges-patents.txt

pagerank-upd: build
	${CMD} "${BINS}/pagerank-update-mapper" "${BINS}/pagerank-update-reducer" pagerank-update ../output/pagerank/part-00000 1

pagerank-small: build
	${CMD} "${BINS}/pagerank-initial-mapper" "${BINS}/pagerank-initial-reducer" pagerank ${SMALL} 1

pagerank: build
	${CMD} "${BINS}/pagerank-initial-mapper" "${BINS}/pagerank-initial-reducer" pagerank ${LARGE} 100

degree-centrality-small: build
	${CMD} "${BINS}/degree-centrality-mapper" "${BINS}/degree-centrality-reducer" degree-centrality ${SMALL}

degree-centrality: build
	${CMD} "${BINS}/degree-centrality-mapper" "${BINS}/degree-centrality-reducer" degree-centrality ${LARGE}

build: format
	cargo build --release

format:
	cargo fmt