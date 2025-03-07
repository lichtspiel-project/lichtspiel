FROM rust:1.85-slim-bookworm

RUN apt-get update && apt-get install -y make curl unzip g++
RUN rustup component add clippy rustfmt