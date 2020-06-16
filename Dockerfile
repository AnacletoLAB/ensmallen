FROM ubuntu:18.04

ENV DEBIAN_FRONTEND=noninteractive
# 32 bit
RUN dpkg --add-architecture i386
# Bare minimum packages to compile C sources
RUN apt-get update -qyy && apt-get install -qyy apt-utils build-essential software-properties-common

###########################################################
# Basic devel tools
###########################################################
RUN  apt-get update -qyy && apt-get install -qyy \
    nano \
    neovim \
    tmux \
    git \
    wget \
    curl \
    cmake \
    unzip \
    gzip \
    pkg-config \
    libstdc++6:i386


###########################################################
# Install anaconda to have an easily reporducible python environments
###########################################################
RUN wget https://repo.anaconda.com/archive/Anaconda3-2020.02-Linux-x86_64.sh -O anaconda.sh
RUN bash ./anaconda.sh -b
RUN echo "export PATH=\$PATH:/root/anaconda3/bin" >> /root/.bashrc
RUN /root/anaconda3/bin/pip install maturin

###########################################################
# Setup Rust nightly
###########################################################
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /root/rustup.sh
RUN chmod +x /root/rustup.sh
RUN /root/rustup.sh --default-host x86_64-unknown-linux-gnu --default-toolchain nightly --profile default -y
RUN echo "source /root/.cargo/env" >> /root/.bashrc
RUN rm /root/rustup.sh

###########################################################
# Build directory
###########################################################
RUN mkdir -p /build
WORKDIR /build