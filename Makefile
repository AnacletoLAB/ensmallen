
BUILD_COMMAND=maturin build --release
MANYLINUX=sudo docker run --rm -v "$${PWD}:/io" manylinux1

python:
	(cd bindings/python; ${BUILD_COMMAND})

python_manylinux1:
	${MANYLINUX} make python

build_manylinux1:
	sudo docker build -t manylinux1 -f ./setup/DockerFileManylinux1 ./setup

update:
	(cd graph; cargo update)
	(cd bindings/python; cargo update)
	(cd fuzzing/graph_harness; cargo update)
	(cd fuzzing/graph_harness/fuzz; cargo update)
	(cd fuzzing/honggfuzz/from_csv; cargo update)
	(cd fuzzing/honggfuzz/from_vec; cargo update)