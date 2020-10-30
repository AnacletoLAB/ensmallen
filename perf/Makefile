first:
	rm -rfd ./target
	cargo update
	(\
		export RUST_FLAGS="";\
		cargo build --release\
	)
	perf record --call-graph=dwarf ./target/release/first

second:
	rm -rfd ./target
	cargo update
	(\
		export RUST_FLAGS="";\
		cargo build --release\
	)
	perf record --call-graph=dwarf ./target/release/second

second_fast:
	rm -rfd ./target
	cargo update
	(\
		export RUST_FLAGS="";\
		cargo build --release\
	)
	perf record --call-graph=dwarf ./target/release/second_fast