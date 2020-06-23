
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
	#rm -rfd target
	# Use a subshell so that we don't modify the user environment-variables
	#cargo test --no-run

	for TEST in $$(find ./target/debug/deps -executable -type f | grep test);\
	do \
		mkdir -p ./target/debug/cov/$$TEST ;\
		kcov --exclude-pattern=/.rustup,/.cargo ./target/debug/cov/$$(basename $$TEST) --verify $$TEST ;\
	done
	
	kcov --merge ./target/debug/cov/total $$(find ./target/debug/cov/ -type d -maxdepth 1 -mindepth 1) ;\
	
	$(OPEN_COMMAND) ./target/debug/cov/total/index.html > /dev/null &

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