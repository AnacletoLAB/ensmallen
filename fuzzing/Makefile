libfuzz_from_csv:
	(\
	cd graph_harness;\
	cargo fuzz run from_csv -- -timeout=10; \
	)

libfuzz_from_vec:
	(\
	cd graph_harness;\
	cargo fuzz run from_vec -- -timeout=10; \
	)

hfuzz_from_csv:
	(\
	cd honggfuzz/from_csv;\
	cargo hfuzz run from_csv; \
	)

hfuzz_from_vec:
	(\
	cd honggfuzz/from_vec;\
	cargo hfuzz run from_vec; \
	)

afl_from_csv:
	(\
	cd afl/from_csv;\
	cargo run --release; \
	)

afl_from_vec:
	(\
	cd afl/from_vec;\
	cargo run --release; \
	)