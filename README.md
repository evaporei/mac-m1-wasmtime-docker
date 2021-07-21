# mac-m1-wasmtime-docker

Project to isolate this [issue](https://github.com/graphprotocol/graph-node/issues/2325) we're having on [`graph-node`](https://github.com/graphprotocol/graph-node) about using [`wasmtime`](https://github.com/bytecodealliance/wasmtime) alongside `docker` on the new Mac M1.

For now it's still working.

To build:

```
docker build -t mac-m1-wasmtime-docker .
```

To run:

```
docker run mac-m1-wasmtime-docker
```
