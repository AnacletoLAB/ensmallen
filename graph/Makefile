
UNAME := $(shell uname)

ifeq ($(UNAME), Linux)
OPEN_COMMAND := "xdg-open"
else
OPEN_COMMAND := "open"
endif


coverage_osx:
	(\
	export CARGO_INCREMENTAL=0;\
	export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort";\
	export RUSTDOCFLAGS="-Cpanic=abort";\
	rm -rfd target;\
	cargo test;\
	mkdir ./target/debug/coverage;\
	grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/;\
	$(OPEN_COMMAND) ./target/debug/coverage/index.html > /dev/null & \
	)
