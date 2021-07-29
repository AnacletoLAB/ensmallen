
UNAME := $(shell uname)

ifeq ($(UNAME), Linux)
OPEN_COMMAND := "xdg-open"
else
OPEN_COMMAND := "open"
endif


coverage:
	(\
	export CARGO_INCREMENTAL=0;\
	export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort";\
	export RUSTDOCFLAGS="-Cpanic=abort";\
	rm -rfd target;\
	rustup update;\
	cargo update;\
	cargo test;\
	mkdir ./target/debug/coverage;\
	grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/;\
	$(OPEN_COMMAND) ./target/debug/coverage/src/index.html > /dev/null & \
	)

test:
	cargo check
	cargo clippy -- -W clippy::pedantic
	(cd ../notebooks_and_scripts/code_analysis/metatest; cargo run --release --bin check)
	cargo test
