.PHONY: pagerank degree-centrality run format

CMD=./tools/hadoop.sh
BINS=./target/release
SMALL=edges-papers.txt
LARGE=edges-patents.txt

pagerank-sum: build
	${CMD} "${BINS}/identity-mapper" "${BINS}/pagerank-combiner" pagerank-sum ../output/pagerank-intermediate-last/part-* "${BINS}/pagerank-combiner"

pagerank-intermediate: build
	${CMD} "${BINS}/identity-mapper" "${BINS}/pagerank-reducer" pagerank-intermediate ../output/pagerank-intermediate-last/part-* "${BINS}/pagerank-combiner"
	if [ -d output/pagerank-intermediate-last ]; then rm -r output/pagerank-intermediate-last; fi;
	mv -f output/pagerank-intermediate output/pagerank-intermediate-last

pagerank-first: build
	${CMD} "${BINS}/pagerank-mapper" "${BINS}/pagerank-reducer" pagerank-intermediate ../output/pagerank-preprocess/part-* "${BINS}/pagerank-combiner"
	if [ -d output/pagerank-intermediate-last ]; then rm -r output/pagerank-intermediate-last; fi;
	mv -f output/pagerank-intermediate output/pagerank-intermediate-last

pagerank-preprocess: build
	${CMD} "${BINS}/identity-mapper" "${BINS}/append-reducer" pagerank-preprocess test-list.txt

degree-centrality: build
	${CMD} "${BINS}/degree-centrality-mapper" "${BINS}/degree-centrality-reducer" degree-centrality test-list.txt

build: format
	cargo build --release

format:
	cargo fmt