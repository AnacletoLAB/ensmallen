
BUILD_COMMAND=maturin build --release
MANYLINUX=sudo docker run --rm -v "$${PWD}:/io" manylinux1

python:
	(cd bindings/python; ${BUILD_COMMAND})

python_manylinux1:
	${MANYLINUX} make python

build_manylinux1:
	sudo docker build -t manylinux1 -f ./setup/Manylinux1 ./setup