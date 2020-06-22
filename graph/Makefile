
UNAME := $(shell uname)

ifeq ($(UNAME), Linux)
OPEN_COMMAND := "xdg-open"
else
OPEN_COMMAND := "open"
endif

doc:
	rm -rfd ./target/doc
	cargo doc --no-deps

coverage:
	rm -rfd target
	# Use a subshell so that we don't modify the user environment-variables
	( export CARGO_INCREMENTAL=0 ;\
		export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" ;\
		export RUSTDOCFLAGS="-Cpanic=abort" ;\
		cargo build ;\
		cargo test ;\
		grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/ ;\
	)
	$(OPEN_COMMAND) ./target/debug/coverage/index.html > /dev/null &

fuzz:
	cargo fuzz run only_edges --release
	cargo fuzz run with_nodes --release

fuzz_coverage:
	cargo fuzz build
	mkdir -p ./fuzz/cov
	kcov --include-path .,.. ./fuzz/cov/edges_cov ./fuzz/target/x86_64-unknown-linux-gnu/debug/only_edges ./fuzz/corpus/only_edges/* || true
	kcov --include-path .,.. ./fuzz/cov/nodes_cov ./fuzz/target/x86_64-unknown-linux-gnu/debug/with_nodes ./fuzz/corpus/with_nodes/* || true
	kcov --merge ./fuzz/cov/total_cov ./fuzz/cov/edges_cov ./fuzz/cov/nodes_cov
	$(OPEN_COMMAND) ./fuzz/cov/total_cov/index.html > /dev/null &

clean_corpus:
	rm $$(find ./fuzz/corpus/only_edges -type f | grep -v .csv | grep -v .tsv | xargs)
	rm $$(find ./fuzz/corpus/with_nodes -type f | grep -v .csv | grep -v .tsv | xargs)