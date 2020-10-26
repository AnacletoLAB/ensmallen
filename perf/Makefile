first:
	rm -rfd ./target
	cargo update
	(\
		export RUST_FLAGS="";\
		cargo build --release\
	)
	perf record --call-graph=dwarf ./target/release/first -o first.dat

second:
	rm -rfd ./target
	cargo update
	(\
		export RUST_FLAGS="";\
		cargo build --release\
	)
	perf record --call-graph=dwarf ./target/release/second -o second.dat