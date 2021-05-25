random_state := 1337420
TIMEOUT := 10 # seconds
MAX_RAM := 8192 # Mb
MAX_LEN := 81920 # bytes
CORPUS_FOLDER := corpus
THREADS := 12


libfuzz_from_csv:
	rm -rfd ./unit_tests/*
	mkdir -p corpus/from_csv
	(\
	cd graph_harness;\
	cargo update;\
	RAYON_NUM_THREADS=2 cargo fuzz run from_csv ../$(CORPUS_FOLDER)/from_csv/ -- -timeout=$(TIMEOUT) -rss_limit_mb=$(MAX_RAM) -max_len=$(MAX_LEN) -random_state=$(random_state); \
	)

libfuzz_from_vec:
	rm -rfd ./unit_tests/*
	mkdir -p corpus/from_vec
	(\
	cd graph_harness;\
	cargo update;\
	RAYON_NUM_THREADS=2 cargo fuzz run from_vec ../$(CORPUS_FOLDER)/from_vec/ -- -timeout=$(TIMEOUT) -rss_limit_mb=$(MAX_RAM) -max_len=$(MAX_LEN) -random_state=$(random_state);\
	)

libfuzz_meta_test:
	rm -rfd ./unit_tests/*
	mkdir -p corpus/meta_test
	(\
	cd graph_harness;\
	cargo update;\
	RAYON_NUM_THREADS=2 cargo fuzz run meta_test ../$(CORPUS_FOLDER)/meta_test/ -- -timeout=$(TIMEOUT) -rss_limit_mb=$(MAX_RAM) -max_len=$(MAX_LEN) -random_state=$(random_state);\
	)

hfuzz_from_csv:
	rm -rfd ./unit_tests/*
	mkdir -p corpus/from_csv
	(\
	cd honggfuzz/from_csv;\
	cargo update;\
	RAYON_NUM_THREADS=2 HFUZZ_INPUT="../../$(CORPUS_FOLDER)/from_csv/" HFUZZ_RUN_ARGS="-t $(TIMEOUT) --threads $(THREADS) --rlimit_rss $(MAX_RAM) --max_file_size $(MAX_LEN) -P" cargo hfuzz run from_csv; \
	)

hfuzz_from_vec:
	rm -rfd ./unit_tests/*
	mkdir -p corpus/from_vec
	(\
	cd honggfuzz/from_vec;\
	cargo update;\
	RAYON_NUM_THREADS=2 HFUZZ_INPUT="../../$(CORPUS_FOLDER)/from_vec/" HFUZZ_RUN_ARGS="-t $(TIMEOUT) --threads $(THREADS) --rlimit_rss $(MAX_RAM) --max_file_size $(MAX_LEN) -P" cargo hfuzz run from_vec; \
	)

hfuzz_meta_test:
	rm -rfd ./unit_tests/*
	mkdir -p corpus/meta_test
	(\
	cd honggfuzz/meta_test;\
	cargo update;\
	RAYON_NUM_THREADS=2 HFUZZ_INPUT="../../$(CORPUS_FOLDER)/meta_test/" HFUZZ_RUN_ARGS="-t $(TIMEOUT) --threads $(THREADS)  --tmout_sigvtalrm --rlimit_rss $(MAX_RAM) --max_file_size $(MAX_LEN) -P" cargo hfuzz run meta_test; \
	)

hfuzz_features:
	rm -rfd ./unit_tests/*
	mkdir -p corpus/features
	(\
	cd honggfuzz/features;\
	cargo update;\
	RAYON_NUM_THREADS=2 HFUZZ_INPUT="../../$(CORPUS_FOLDER)/features/" HFUZZ_RUN_ARGS="-t $(TIMEOUT) --threads $(THREADS)  --tmout_sigvtalrm --rlimit_rss $(MAX_RAM) --max_file_size $(MAX_LEN) -P" cargo hfuzz run features; \
	)

hfuzz_builder:
	rm -rfd ./unit_tests/*
	mkdir -p corpus/builder
	(\
	cd honggfuzz/builder;\
	cargo update;\
	RAYON_NUM_THREADS=2 HFUZZ_INPUT="../../$(CORPUS_FOLDER)/builder/" HFUZZ_RUN_ARGS="-t $(TIMEOUT) --threads $(THREADS) --rlimit_rss $(MAX_RAM) --max_file_size $(MAX_LEN) -P" cargo hfuzz run builder; \
	)

hfuzz_components:
	rm -rfd ./unit_tests/*
	mkdir -p corpus/components
	(\
	cd honggfuzz/components;\
	cargo update;\
	RAYON_NUM_THREADS=2 HFUZZ_INPUT="../../$(CORPUS_FOLDER)/components/" HFUZZ_RUN_ARGS="-t $(TIMEOUT) --threads $(THREADS) --rlimit_rss $(MAX_RAM) --max_file_size $(MAX_LEN) -P" cargo hfuzz run components; \
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
