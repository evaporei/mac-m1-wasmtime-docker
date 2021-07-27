# mac-m1-wasmtime-docker

Project to isolate this [issue](https://github.com/graphprotocol/graph-node/issues/2325) we're having on [`graph-node`](https://github.com/graphprotocol/graph-node) about using [`wasmtime`](https://github.com/bytecodealliance/wasmtime) alongside `docker` on the new Mac M1.

To build:

```
docker build -t mac-m1-wasmtime-docker .
```

Right now the build is failing like this since commit 211628df157b267b60879bdbed2e2adb8ec8ced3:

```
#8 198.0    Compiling cranelift-entity v0.74.0
#8 198.1    Compiling cranelift-codegen-shared v0.74.0
#8 198.3    Compiling regalloc v0.0.31
#8 198.4    Compiling toml v0.5.8
#8 198.4    Compiling bincode v1.3.3
#8 198.5    Compiling gimli v0.24.0
#8 199.0    Compiling object v0.24.0
#8 206.6    Compiling cranelift-bforest v0.74.0
#8 206.9    Compiling addr2line v0.15.2
#8 206.9    Compiling cranelift-codegen-meta v0.74.0
#8 215.9    Compiling cranelift-codegen v0.74.0
#8 220.7    Compiling zstd v0.6.1+zstd.1.4.9
#8 225.2    Compiling cranelift-frontend v0.74.0
#8 225.2    Compiling cranelift-native v0.74.0
#8 225.9    Compiling cranelift-wasm v0.74.0
#8 228.6    Compiling wasmtime-environ v0.27.0
#8 235.9    Compiling wasmtime-debug v0.27.0
#8 236.2    Compiling wasmtime-cranelift v0.27.0
#8 241.6    Compiling wasmtime-profiling v0.27.0
#8 246.4    Compiling wasmtime-obj v0.27.0
#8 249.0    Compiling wasmtime-jit v0.27.0
#8 250.0    Compiling wasmtime v0.27.0
#8 2499.7 error: could not compile `cranelift-codegen`
#8 2499.7
#8 2499.7 Caused by:
#8 2499.9   process didn't exit successfully: `rustc --crate-name cranelift_codegen --edition=2018 /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/cranelift-codegen-0.74.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no --cfg 'feature="default"' --cfg 'feature="enable-serde"' --cfg 'feature="gimli"' --cfg 'feature="serde"' --cfg 'feature="std"' --cfg 'feature="unwind"' -C metadata=9b14cee6cb41e616 -C extra-filename=-9b14cee6cb41e616 --out-dir /mac-m1-wasmtime-docker/target/release/deps -L dependency=/mac-m1-wasmtime-docker/target/release/deps --extern cranelift_bforest=/mac-m1-wasmtime-docker/target/release/deps/libcranelift_bforest-370ea9f2a0e1ee8a.rmeta --extern cranelift_codegen_shared=/mac-m1-wasmtime-docker/target/release/deps/libcranelift_codegen_shared-9d034acf9a6c9df5.rmeta --extern cranelift_entity=/mac-m1-wasmtime-docker/target/release/deps/libcranelift_entity-73a6e686e40b31bc.rmeta --extern gimli=/mac-m1-wasmtime-docker/target/release/deps/libgimli-6133774468fc4d17.rmeta --extern log=/mac-m1-wasmtime-docker/target/release/deps/liblog-5c520fdec0c7cc85.rmeta --extern regalloc=/mac-m1-wasmtime-docker/target/release/deps/libregalloc-5c9a3426f5eb49e4.rmeta --extern serde=/mac-m1-wasmtime-docker/target/release/deps/libserde-73cfd607f615fa8f.rmeta --extern smallvec=/mac-m1-wasmtime-docker/target/release/deps/libsmallvec-f0a040164e35718f.rmeta --extern target_lexicon=/mac-m1-wasmtime-docker/target/release/deps/libtarget_lexicon-c3812c0b417b569d.rmeta --cap-lints allow -g --cfg 'feature="arm64"'` (signal: 9, SIGKILL: kill)
#8 2499.9 warning: build failed, waiting for other jobs to finish...
#8 2515.1 error: failed to compile `mac-m1-wasmtime-docker v0.1.0 (/mac-m1-wasmtime-docker)`, intermediate artifacts can be found at `/mac-m1-wasmtime-docker/target`
#8 2515.1
#8 2515.1 Caused by:
#8 2515.1   build failed
------
executor failed running [/bin/sh -c RUSTFLAGS="-g" cargo install --locked --path .   && cargo clean   && objcopy --only-keep-debug /usr/local/cargo/bin/mac-m1-wasmtime-docker /usr/local/cargo/bin/mac-m1-wasmtime-docker.debug   && strip -g /usr/local/cargo/bin/mac-m1-wasmtime-docker   && cd /usr/local/cargo/bin   && objcopy --add-gnu-debuglink=mac-m1-wasmtime-docker.debug mac-m1-wasmtime-docker]: exit code: 101
```

To run:

```
docker run mac-m1-wasmtime-docker
```
