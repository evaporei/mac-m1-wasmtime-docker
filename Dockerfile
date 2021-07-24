FROM rust:latest

WORKDIR /mac-m1-wasmtime-docker

COPY . .

RUN RUSTFLAGS="-g" cargo install --locked --path . \
  && cargo clean \
  && objcopy --only-keep-debug /usr/local/cargo/bin/mac-m1-wasmtime-docker /usr/local/cargo/bin/mac-m1-wasmtime-docker.debug \
  && strip -g /usr/local/cargo/bin/mac-m1-wasmtime-docker \
  && cd /usr/local/cargo/bin \
  && objcopy --add-gnu-debuglink=mac-m1-wasmtime-docker.debug mac-m1-wasmtime-docker

CMD ["mac-m1-wasmtime-docker"]
