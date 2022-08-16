A fork of [gifski, from ImageOptim](https://github.com/ImageOptim/gifski) to include bindings for Python.

# Installing from source
```
# install Rust from https://rustup.rs/
# You may need to restart your shell/terminal for environment changes to take effect

# Set up maturin
python3 -m venv .venv
source .venv/bin/activate
pip install maturin

# Option 1: build the wheel
maturin build

# Option 2: install the library
maturin develop
```