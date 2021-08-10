random_state := 1337420
TIMEOUT := 60 # seconds
MAX_RAM := 8192 # Mb
MAX_LEN := 81920 # bytes
CORPUS_FOLDER := corpus
THREADS := "`python -c 'import multiprocessing as mp; print(int(0.5 * mp.cpu_count()))'`"


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

hfuzz_meta_test:
	rm -rfd ./unit_tests/*
	mkdir -p corpus/meta_test
	(\
	cd honggfuzz;\
	cargo update;\
	RAYON_NUM_THREADS=2 HFUZZ_INPUT="../$(CORPUS_FOLDER)/meta_test/" HFUZZ_RUN_ARGS="-t $(TIMEOUT) --threads $(THREADS)  --tmout_sigvtalrm --rlimit_rss $(MAX_RAM) --max_file_size $(MAX_LEN) -P" cargo hfuzz run meta_test; \
	)

hfuzz_from_strings:
	rm -rfd ./unit_tests/*
	mkdir -p corpus/from_strings
	(\
	cd honggfuzz;\
	cargo update;\
	RAYON_NUM_THREADS=2 HFUZZ_INPUT="../$(CORPUS_FOLDER)/from_strings/" HFUZZ_RUN_ARGS="-t $(TIMEOUT) --threads $(THREADS)  --tmout_sigvtalrm --rlimit_rss $(MAX_RAM) --max_file_size $(MAX_LEN) -P" cargo hfuzz run from_strings; \
	)