# Bindings
Being written in Rust it's possible to expose this module to various languages and frameworks such as Python,
Matlab, R, Java, C, ..

## Python bindings
This project is currently work in progress, and is to be considered for all
intents and porposes an **alpha** version.

To install the **latest (alpha) release**, run the following:

```bash
pip install ensmallen
```

The pre-compiled wheels needs glibc >= 2.12.

To build it locally running the following command should be enough.
```bash
make python
```

See [this page](https://github.com/LucaCappelletti94/ensmallen/blob/master/bindings/python/README.md) to compile the bindings yourself.