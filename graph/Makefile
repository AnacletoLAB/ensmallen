
UNAME := $(shell uname)

ifeq ($(UNAME), Linux)
OPEN_COMMAND := "xdg-open"
else
OPEN_COMMAND := "open"
endif

coverage:
	rm -rfd target
	cargo build
	cargo test
	grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/
	$(OPEN_COMMAND) ./target/debug/coverage/index.html

fuzz:
	cargo fuzz run only_edge_file --release

fuzz_coverage:
	kcov --include-path .,.. ./fuzz/cov ./fuzz/target/x86_64-unknown-linux-gnu/debug/only_edge_file ./fuzz/corpus/only_edge_file/*
	$(OPEN_COMMAND) ./fuzz/cov/index.html