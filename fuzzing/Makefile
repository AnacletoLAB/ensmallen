TIMEOUT := "10"
CORPUS_FOLDER := corpus


libfuzz_from_csv:
	(\
	cd graph_harness;\
	cargo fuzz run from_csv ../$(CORPUS_FOLDER)/from_csv/ -- -timeout=$(TIMEOUT); \
	)

libfuzz_from_vec:
	(\
	cd graph_harness;\
	cargo fuzz run from_vec ../$(CORPUS_FOLDER)/from_vec/ -- -timeout=$(TIMEOUT); \
	)

hfuzz_from_csv:
	(\
	cd honggfuzz/from_csv;\
	HFUZZ_INPUT="../../$(CORPUS_FOLDER)/from_csv/" HFUZZ_RUN_ARGS="-t $(TIMEOUT) -P --exit_upon_crash" RUSTFLAGS="-Z sanitizer=address" cargo hfuzz run from_csv; \
	)

hfuzz_from_vec:
	(\
	cd honggfuzz/from_vec;\
	HFUZZ_INPUT="../../$(CORPUS_FOLDER)/from_vec/" HFUZZ_RUN_ARGS="-t $(TIMEOUT) -P --exit_upon_crash" RUSTFLAGS="-Z sanitizer=address" cargo hfuzz run from_vec; \
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