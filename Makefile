
BUILD_COMMAND=make build
MANYLINUX=sudo docker run --rm -v "$${PWD}:/io" manylinux2010

python:
	(cd bindings/python; ${BUILD_COMMAND})

python_manylinux2010:
	${MANYLINUX} make python

build_manylinux2010:
	sudo docker build -t manylinux2010 -f ./setup/DockerFileManylinux2010 ./setup



update:
	(cd graph; cargo update)
	(cd bindings/python; cargo update)
	(cd fuzzing/graph_harness; cargo update)
	(cd fuzzing/graph_harness/fuzz; cargo update)
	(cd fuzzing/honggfuzz/from_csv; cargo update)
	(cd fuzzing/honggfuzz/from_vec; cargo update)
	(cd fuzzing/afl/from_csv; cargo update)
	(cd fuzzing/afl/from_vec; cargo update)




bindgen:
	(cd code_analysis; cargo run --release --bin bindgen)

check:
	(cd code_analysis; cargo run --release --bin check)

build_metatest_harness:
	(cd code_analysis; cargo run --release --bin metatest)




test: test_from_vec test_meta_test test_graph check # test_from_csv 

test_graph:
	(cd graph; cargo test --release)

test_from_csv:
	$(MAKE) -C "./fuzzing/stupid_fuzzer" test_from_csv

test_from_vec:
	$(MAKE) -C "./fuzzing/stupid_fuzzer" test_from_vec

test_meta_test:
	$(MAKE) -C "./fuzzing/stupid_fuzzer" test_meta_test




hfuzz_from_csv:
	$(MAKE) -C "./fuzzing" hfuzz_from_csv

hfuzz_from_vec:
	$(MAKE) -C "./fuzzing" hfuzz_from_vec

hfuzz_meta_test:
	$(MAKE) -C "./fuzzing" hfuzz_meta_test