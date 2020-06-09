coverage:
	rm -rfd target
	cargo build
	cargo test
	grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/
	open ./target/debug/coverage/index.html