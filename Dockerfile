FROM ubuntu:16.04

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
    vim \
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
# Setup different python versions
###########################################################

RUN  apt-get update -qyy && apt-get install -qyy zlib1g-dev

RUN wget https://www.python.org/ftp/python/3.8.3/Python-3.8.3.tar.xz
RUN tar -xf Python-3.8.3.tar.xz
RUN (cd Python-3.8.3; ./configure --enable-optimizations; make -j$(nproc); make -j$(nproc) install)

RUN wget https://www.python.org/ftp/python/3.7.7/Python-3.7.7.tar.xz
RUN tar -xf Python-3.7.7.tar.xz
RUN (cd Python-3.7.7; ./configure --enable-optimizations; make -j$(nproc); make -j$(nproc) install)

RUN wget https://www.python.org/ftp/python/3.6.10/Python-3.6.10.tar.xz
RUN tar -xf Python-3.6.10.tar.xz
RUN (cd Python-3.6.10; ./configure --enable-optimizations; make -j$(nproc); make -j$(nproc) install)

RUN wget https://www.python.org/ftp/python/3.5.9/Python-3.5.9.tar.xz
RUN tar -xf Python-3.5.9.tar.xz
RUN (cd Python-3.5.9; ./configure --enable-optimizations; make -j$(nproc); make -j$(nproc) install)


###########################################################
# Build directory
###########################################################
RUN mkdir -p /build
WORKDIR /build