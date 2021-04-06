random_state := 1337420
TIMEOUT := 60 # seconds
MAX_RAM := 8192 # Mb
MAX_LEN := 81920 # bytes
CORPUS_FOLDER := corpus


libfuzz_from_csv:
	rm -rfd ./unit_tests/*
	(\
	cd graph_harness;\
	cargo update;\
	cargo fuzz run from_csv ../$(CORPUS_FOLDER)/from_csv/ -- -timeout=$(TIMEOUT) -rss_limit_mb=$(MAX_RAM) -max_len=$(MAX_LEN) -random_state=$(random_state); \
	)

libfuzz_from_vec:
	rm -rfd ./unit_tests/*
	(\
	cd graph_harness;\
	cargo update;\
	cargo fuzz run from_vec ../$(CORPUS_FOLDER)/from_vec/ -- -timeout=$(TIMEOUT) -rss_limit_mb=$(MAX_RAM) -max_len=$(MAX_LEN) -random_state=$(random_state);\
	)

libfuzz_mega_test:
	rm -rfd ./unit_tests/*
	(\
	cd graph_harness;\
	cargo update;\
	cargo fuzz run mega_test ../$(CORPUS_FOLDER)/mega_test/ -- -timeout=$(TIMEOUT) -rss_limit_mb=$(MAX_RAM) -max_len=$(MAX_LEN) -random_state=$(random_state);\
	)


hfuzz_from_csv:
	rm -rfd ./unit_tests/*
	(\
	cd honggfuzz/from_csv;\
	cargo update;\
	HFUZZ_INPUT="../../$(CORPUS_FOLDER)/from_csv/" HFUZZ_RUN_ARGS="-t $(TIMEOUT) --threads 16 --rlimit_rss $(MAX_RAM) --max_file_size $(MAX_LEN) -P --exit_upon_crash" RUSTFLAGS="-Z sanitizer=address" cargo hfuzz run from_csv; \
	)

hfuzz_from_vec:
	rm -rfd ./unit_tests/*
	(\
	cd honggfuzz/from_vec;\
	cargo update;\
	HFUZZ_INPUT="../../$(CORPUS_FOLDER)/from_vec/" HFUZZ_RUN_ARGS="-t $(TIMEOUT) --threads 64  --rlimit_rss $(MAX_RAM) --max_file_size $(MAX_LEN) -P --exit_upon_crash" RUSTFLAGS="-Z sanitizer=address" cargo hfuzz run from_vec; \
	)

hfuzz_mega_test:
	rm -rfd ./unit_tests/*
	(\
	cd honggfuzz/mega_test;\
	cargo update;\
	HFUZZ_INPUT="../../$(CORPUS_FOLDER)/mega_test/" HFUZZ_RUN_ARGS="-t $(TIMEOUT) --threads 64  --rlimit_rss $(MAX_RAM) --max_file_size $(MAX_LEN) -P --exit_upon_crash" RUSTFLAGS="-Z sanitizer=address" cargo hfuzz run mega_test; \
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
