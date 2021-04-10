random_state := 1337420
TIMEOUT := 60 # seconds
MAX_RAM := 8192 # Mb
MAX_LEN := 81920 # bytes
CORPUS_FOLDER := corpus
THREADS := 12


libfuzz_from_csv:
	rm -rfd ./unit_tests/*
	(\
	cd graph_harness;\
	cargo update;\
	RAYON_NUM_THREADS=2 cargo fuzz run from_csv ../$(CORPUS_FOLDER)/from_csv/ -- -timeout=$(TIMEOUT) -rss_limit_mb=$(MAX_RAM) -max_len=$(MAX_LEN) -random_state=$(random_state); \
	)

libfuzz_from_vec:
	rm -rfd ./unit_tests/*
	(\
	cd graph_harness;\
	cargo update;\
	RAYON_NUM_THREADS=2 cargo fuzz run from_vec ../$(CORPUS_FOLDER)/from_vec/ -- -timeout=$(TIMEOUT) -rss_limit_mb=$(MAX_RAM) -max_len=$(MAX_LEN) -random_state=$(random_state);\
	)

hfuzz_from_csv:
	rm -rfd ./unit_tests/*
	(\
	cd honggfuzz/from_csv;\
	cargo update;\
	RAYON_NUM_THREADS=2 HFUZZ_INPUT="../../$(CORPUS_FOLDER)/from_csv/" HFUZZ_RUN_ARGS="-t $(TIMEOUT) --threads $(THREADS) --rlimit_rss $(MAX_RAM) --max_file_size $(MAX_LEN) -P --exit_upon_crash" RUSTFLAGS="-Z sanitizer=address" cargo hfuzz run from_csv; \
	)

hfuzz_from_vec:
	rm -rfd ./unit_tests/*
	(\
	cd honggfuzz/from_vec;\
	cargo update;\
	RAYON_NUM_THREADS=2 HFUZZ_INPUT="../../$(CORPUS_FOLDER)/from_vec/" HFUZZ_RUN_ARGS="-t $(TIMEOUT) --threads $(THREADS) --rlimit_rss $(MAX_RAM) --max_file_size $(MAX_LEN) -P --exit_upon_crash" RUSTFLAGS="-Z sanitizer=address" cargo hfuzz run from_vec; \
	)

afl_from_csv:
	rm -rfd ./unit_tests/*
	(\
	cd afl/from_csv;\
	cargo update;\
	cargo run --release; \
	)

afl_from_vec:
	rm -rfd ./unit_tests/*
	(\
	cd afl/from_vec;\
	cargo update;\
	cargo run --release; \
	)
