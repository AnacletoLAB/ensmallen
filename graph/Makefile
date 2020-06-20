
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