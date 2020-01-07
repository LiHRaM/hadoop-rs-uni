.PHONY: pagerank degree-centrality run format

CMD=./tools/hadoop.sh
BINS=./target/release

pagerank:
	./tools/pagerank.sh

pagerank-sum:
	${CMD} "${BINS}/identity-mapper" "${BINS}/pagerank-combiner" pagerank-sum ../output/pagerank-intermediate-last/part-* "${BINS}/pagerank-combiner"

pagerank-intermediate:
	${CMD} "${BINS}/identity-mapper" "${BINS}/pagerank-reducer" pagerank-intermediate ../output/pagerank-intermediate-last/part-* "${BINS}/pagerank-combiner"
	if [ -d output/pagerank-intermediate-last ]; then rm -r output/pagerank-intermediate-last; fi;
	mv -f output/pagerank-intermediate output/pagerank-intermediate-last

pagerank-first:
	${CMD} "${BINS}/pagerank-mapper" "${BINS}/pagerank-reducer" pagerank-intermediate ../output/pagerank-preprocess/part-* "${BINS}/pagerank-combiner"
	if [ -d output/pagerank-intermediate-last ]; then rm -r output/pagerank-intermediate-last; fi;
	mv -f output/pagerank-intermediate output/pagerank-intermediate-last

pagerank-preprocess: build
	${CMD} "${BINS}/identity-mapper" "${BINS}/append-reducer" pagerank-preprocess ${FILE}

degree-centrality: build
	${CMD} "${BINS}/degree-centrality-mapper" "${BINS}/degree-centrality-reducer" degree-centrality ${FILE}

build: format
	cargo build --release

format:
	cargo fmt