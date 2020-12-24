
BUILD_COMMAND=maturin build --release
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
