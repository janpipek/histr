test-rust:
    cargo test

build:
    maturin build

ipython:
    maturin develop --uv
    uv sync
    uv run ipython

clean:
    rm -rf target
