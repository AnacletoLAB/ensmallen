FROM tensorflow/tensorflow:latest-gpu-jupyter 

################################################################################
# Setup the environment
################################################################################
# Install common tools
RUN apt-get update -qyy && \
    apt-get install -qyy \
    build-essential\
    binutils-dev  \
    libunwind-dev \
    libblocksruntime-dev \
    liblzma-dev \
    libnuma-dev \
    wget git curl tmux htop nano vim make

# Install rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y 
# setup the environment variables
ENV PATH /root/.cargo/bin:$PATH
# Install rust nightly (this is needed by maturin)
RUN rustup default nightly
# Update rust to the latest version
RUN rustup update
# Install maturin to build the python bindings for rust
RUN cargo install maturin

# Update pip otherwise it will says that the ensmallen's wheels is not supported
# on this platform
RUN pip install --upgrade pip

################################################################################
# Install ensmallen
################################################################################
# (Future) Install form pypi
#RUN pip install ensmallen
# Clone ensmallen
RUN cd / && git clone https://github.com/AnacletoLAB/ensmallen.git
# Checkout the latest branch
RUN cd /ensmallen && git checkout transitivity
# Update all the packages in all the crates
RUN cd /ensmallen && make update
# Generate the bindings
RUN cd /ensmallen && make bindgen
# Build the bindings
RUN cd /ensmallen/bindings/python && maturin build --release
# Install the bindings 
RUN pip install /ensmallen/bindings/python/target/wheels/ensmallen-0.6.0-cp36-cp36m-manylinux_2_24_x86_64.whl

################################################################################
# Install embiggen
################################################################################
# (Future) Install from pypi
# RUN pip install embiggen
# Clone ensmallen
RUN cd / && git clone https://github.com/monarch-initiative/embiggen.git
# Checkout the latest branch
RUN cd /embiggen && git checkout graph_convolutional_network
# Install 
RUN cd /embiggen && pip install . -U --user

# not needed but we use it often in the experiments scripts
RUN pip install silence_tensorflow