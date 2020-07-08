.PHONY: all test clean
UNAME := $(shell uname)

ifeq ($(UNAME), Linux)
OPEN_COMMAND := "xdg-open"
else
OPEN_COMMAND := "open"
endif

doc:
	rm -rfd ./target/doc
	cargo doc --no-deps

coverage_osx:
	rm -rfd target
	cargo build
	cargo test
	grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/
	$(OPEN_COMMAND) ./target/debug/coverage/index.html > /dev/null &

coverage:
	#rm -rfd target
	# Use a subshell so that we don't modify the user environment-variables
	#cargo test --no-run

	for TEST in $$(find ./target/debug/deps -perm +0111 -type f | grep test);\
	do \
		mkdir -p ./target/debug/cov/$$TEST ;\
		kcov --exclude-pattern=/.rustup,/.cargo ./target/debug/cov/$$(basename $$TEST) --verify $$TEST ;\
	done
	
	kcov --merge ./target/debug/cov/total $$(find ./target/debug/cov/ -type d -maxdepth 1 -mindepth 1) ;\
	
	$(OPEN_COMMAND) ./target/debug/cov/total/index.html > /dev/null &

fuzz:
	RUST_BACKTRACE=1 cargo fuzz run from_csv --release -- -rss_limit_mb=0 -seed=1337

test:
	rm -fdr target
	cargo test

honggfuzz:
	(cd honggfuzz; cargo hfuzz run honggfuzz) 

fuzz_coverage:
	cargo fuzz build
	mkdir -p ./fuzz/cov
	kcov --include-path .,.. ./fuzz/cov/from_csv_cov ./fuzz/target/x86_64-unknown-linux-gnu/debug/from_csv ./fuzz/corpus/from_csv/* || true
	$(OPEN_COMMAND) ./fuzz/cov/from_csv_cov/index.html > /dev/null &

clean_corpus:
	rm $$(find ./fuzz/corpus/only_edges -type f | grep -v .csv | grep -v .tsv | xargs)
	rm $$(find ./fuzz/corpus/with_nodes -type f | grep -v .csv | grep -v .tsv | xargs)
