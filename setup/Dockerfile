FROM tensorflow/tensorflow:latest-gpu-jupyter 

# Install the common tools
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
ENV PATH /root/.cargo/bin:$PATH
RUN rustup default nightly
RUN rustup update

# Install maturin to build the python bindings for rust
RUN cargo install maturin

################################################################################
# Install ensmallen
################################################################################
# (Future) Install form pypi
#RUN pip install ensmallen_graph
# Clone ensmallen
RUN cd ~ && git clone https://github.com/AnacletoLAB/ensmallen_graph.git
# Checkout the latest branch
RUN cd ~/ensmallen_graph && git checkout transitivity
# Update all the packages in all the crates
RUN cd ~/ensmallen_graph && make update
# Build and install the bindings
RUN cd ~/ensmallen_graph/bindings/python && maturin develop --release

################################################################################
# Install embiggen
################################################################################
# (Future) Install from pypi
# RUN pip install embiggen
# Clone ensmallen
RUN cd ~ && git clone https://github.com/monarch-initiative/embiggen.git
# Checkout the latest branch
RUN cd ~/embiggen && git checkout graph_convolutional_networks
# Install 
RUN cd ~/embiggen && pip install . -U --user