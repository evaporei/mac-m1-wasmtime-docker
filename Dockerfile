FROM rust:latest

WORKDIR /mac-m1-wasmtime-docker

COPY . .

RUN cargo install --path .

CMD ["mac-m1-wasmtime-docker"]
