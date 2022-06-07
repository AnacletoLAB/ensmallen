To compile you need to install the target:

```bash
rustup target add nvptx64-nvidia-cuda
```

Do install the `ptx linker` by running:

```bash
cargo install ptx-linker -f --version ">= 0.9"
```

and then you can create the PTX using 

```bash
cargo build --release --target=nvptx64-nvidia-cuda
```