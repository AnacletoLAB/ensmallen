fuzz_from_csv:
	(\
	cd graph_harness;\
	cargo fuzz run from_csv ../corpus/from_csv; \
	)

fuzz_form_vec:
	(\
	cd graph_harness;\
	cargo fuzz run from_vec ../corpus/from_vec; \
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