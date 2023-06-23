# Install qemu and binfmt and then run:
# `docker run --rm --privileged multiarch/qemu-user-static:register --reset`
# To be able to run ARM containers on x86_64
FROM multiarch/ubuntu-debootstrap:arm64-jammy

RUN apt-get update -qy && apt-get install -qy software-properties-common
RUN add-apt-repository universe
RUN apt-get install -qyy \
    build-essential \
    cmake \
    git \
    python3 \
    python3-dev \
    python3-pip \
    python3-setuptools \
    wget \
    curl \
    pkg-config \
    libssl-dev \
    librust-openssl-sys-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN python3 -m pip install --upgrade pip

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh /dev/stdin -y
RUN . $HOME/.cargo/env && rustup default nightly
RUN pip3 install "maturin[zig]"

ENV PKG_CONFIG_PATH /usr/lib/aarch64-linux-gnu/pkgconfig/

