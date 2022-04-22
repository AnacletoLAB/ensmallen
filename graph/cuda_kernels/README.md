To compile you need to install the target:
```
rustup target add nvptx64-nvidia-cuda
```

and then you can create the PTX using 
```
cargo build --release --target=nvptx64-nvidia-cuda
```