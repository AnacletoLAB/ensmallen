# EnsmallenGraph
Rust library to run weighted random walks on very big graphs.

## Setup for Google cloud
Run the following to setup a google cloud machine.
In the future, this should be moved into a docker.

```bash
sudo apt install wget git-all -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
```