default:
    just --list

test-rust:
    cargo test

build:
    maturin build

ipython:
    uv sync
    maturin develop --uv
    uv run ipython

clean:
    rm -rf target

test-python:
    uv run pytest

test: test-rust test-python
