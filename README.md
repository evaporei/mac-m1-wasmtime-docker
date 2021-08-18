# mac-m1-wasmtime-docker

Project to isolate [this issue](https://github.com/graphprotocol/graph-node/issues/2325) we're having on [`graph-node`](https://github.com/graphprotocol/graph-node) about using [`wasmtime`](https://github.com/bytecodealliance/wasmtime) alongside `docker` on the new Mac M1.

The problem happens because we build the image on a linux similarly to this:

```
docker build -t mac-m1-wasmtime-docker .
```

And get the error running it on a Mac M1, failing like this:

```
otaviopace@Otavios-MacBook-Pro ~> docker run d728872e2d2d
WARNING: The requested image's platform (linux/amd64) does not match the detected host platform (linux/arm64/v8) and no specific platform was requested
Initializing...
Compiling module...
Creating callback...
Instantiating module...
Extracting export...
thread 'cool thread name' panicked at 'assertion failed: `(left == right)`
  left: `-1`,
 right: `0`: registering new sigaltstack failed', /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/wasmtime-runtime-0.27.0/src/traphandlers/unix.rs:238:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
result: Err(Any { .. })
```

To reproduce the error you can either save your Docker image (built on a linux) on Docker Hub and pull it on a Mac M1, or you can use [this method](https://stackoverflow.com/questions/24482822/how-to-share-my-docker-image-without-using-the-docker-hub/48856787#48856787).
