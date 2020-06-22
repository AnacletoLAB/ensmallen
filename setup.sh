#!/bin/bash
CURRENT_FOLDER=$(dirname "$(python -c "import os,sys; print(os.path.realpath(sys.argv[1]))" $0)")

# gcc, make, ecc
sudo apt-get update -qyy
sudo apt-get install build-essential htop kcov -qyy

###########################################################
# Install anaconda to have an easily reporducible python environments
###########################################################
wget https://repo.anaconda.com/archive/Anaconda3-2020.02-Linux-x86_64.sh -O anaconda.sh
bash ./anaconda.sh -b
echo "export PATH=\$PATH:$HOME/anaconda3/bin" >> $HOME/.bashrc
$HOME/anaconda3/bin/pip install maturin
$HOME/anaconda3/bin/pip install -r $CURRENT_FOLDER/requirements.txt

###########################################################
# Setup Rust nightly
###########################################################
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > $HOME/rustup.sh
chmod +x /$HOME/rustup.sh
$HOME/rustup.sh --default-host x86_64-unknown-linux-gnu --default-toolchain nightly --profile default -y
echo "source $HOME/.cargo/env" >> $HOME/.bashrc
rm $HOME/rustup.sh

# load for current shell
source $HOME/.bashrc
source $HOME/.cargo/env

# Setup cargo-fuzz
cargo install cargo-fuzz