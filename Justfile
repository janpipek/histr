test:
    cargo test

build:
    maturin build

ipython:
    maturin develop --uv
    uv run ipython