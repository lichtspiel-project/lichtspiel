FROM rust:1.85-slim-bookworm AS development

RUN apt-get update && apt-get install -y make curl unzip g++ bsdmainutils
RUN rustup component add clippy rustfmt

FROM development AS performance-test

COPY .  /app
WORKDIR /app
RUN apt-get install -y bsdmainutils && cargo install hyperfine && cargo install --path .

# test if output is really random
# > rng | hexdump -Cv | head
#
# compare rng and xorshiftstar
# > hyperfine --warmup 3 'rng | head -n 999999' 'xorshiftstar | head -n 999999'