FROM rust:1.85-slim-bookworm AS development

RUN apt-get update && apt-get install -y make curl unzip g++ bsdmainutils
RUN rustup component add clippy rustfmt

FROM development AS performance-test

COPY .  /app
WORKDIR /app
RUN apt-get install -y bsdmainutils && cargo install hyperfine && cargo install --path .

WORKDIR /tmp/pr
COPY ./scripts/* .

RUN bash practrand.sh

WORKDIR /app