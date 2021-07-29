build:
	rm -fdr target
	rustup update
	cargo update
	(cd ../../code_analysis; cargo run --release --bin bindgen)
	RUSTFLAGS="-C opt-level=3 -C target-cpu=native -C inline-threshold=1000" maturin build --release

install:
	pip install --upgrade --user --no-index --find-links=target/wheels ensmallen_graph

test:
	pytest -s ./pytests

build_aarch64:
	# Install toolchain
	yay -S aarch64-unknown-linux-gnu
	# in docker
	sudo docker run -it -v"${PWD}:/src"  rust:latest bash
	apt-get update
	apt-get install gcc-aarch64-linux-gnu
	rustup default nightly
	rustup target add aarch64-unknown-linux-gnu
	rustup toolchain install nightly-aarch64-unknown-linux-gnu
	cargo install maturin

	# Compule python 3.9
	wget ....
	./configure --enable-optimizations --build=x86_64-linux-gnu --host=aarch64-linux-gnu CC="aarch64-linux-gnu-gcc" --disable-ipv6 ac_cv_file__dev_ptmx=no ac_cv_file__dev_ptc=no ac_cv_have_long_long_format=yes
	make -j$(nproc)

	# Cross compyle
	export PYO3_CROSS_PYTHON_VERSION=3.9 
	export PYO3_CROSS="1" 
	export PYO3_CROSS_LIB_DIR="/src/home/zom/Downloads/Python-3.9.6/" 
	export PYO3_CROSS_INCLUDE_DIR="/src/home/zom/Downloads/Python-3.9.6/Include" 
	export RUSTFLAGS="-C linker=aarch64-linux-gnu-gcc" 
	maturin build --target aarch64-unknown-linux-gnu

	#extract in a folder and start python there
	pip install compress_json pandas tqdm downloaders