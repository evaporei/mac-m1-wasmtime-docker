//! Small example of how to instantiate a wasm module that imports one function,
//! showing how you can fill in host functionality for a wasm module.

// You can execute this example with `cargo run --example hello`

use anyhow::Result;
use wasmtime::*;
use std::thread;

fn runtime() -> Result<()> {
    // let (result_sender, result_receiver): (Sender<Result<BlockState<C>, MappingError>>, _) = channel();
    // Configure the initial compilation environment, creating the global
    // `Store` structure. Note that you can also tweak configuration settings
    // with a `Config` and an `Engine` if desired.
    println!("Initializing...");
    let mut config = Config::new();
    config.strategy(Strategy::Cranelift).unwrap();
    config.interruptable(true); // For timeouts.
    config.cranelift_nan_canonicalization(true); // For NaN determinism.
    config.cranelift_opt_level(OptLevel::None);

    let engine = Engine::new(&config)?;

    let store = Store::new(&engine);

    let mut linker = Linker::new(&store);
    linker.func("host", "hello", || println!("hello"))?;

    // let wat = r#"
    // (module
    //     (import "host" "double" (func $double (param i32) (result i32)))
    //     (func (export "double") call $double)
    // )
    // "#;
    let wat = r#"
      (module
        (func $hello (import "host" "hello"))
        (func (export "run") (call $hello))
      )
    "#;
    let module = Module::new(store.engine(), wat)?;
    let instance = linker.instantiate(&module)?;

    // Compile the wasm binary into an in-memory instance of a `Module`.
    println!("Compiling module...");
    // WAT FILE
    // let module = Module::from_file(&engine, "wasm/hello.wat")?;

    // WASM FILE
    // let binary = std::fs::read("wasm/hello.wasm")?;
    // let module = Module::from_binary(&engine, &binary)?;

    // Here we handle the imports of the module, which in this case is our
    // `HelloCallback` type and its associated implementation of `Callback.
    println!("Creating callback...");
    // let hello_func = Func::wrap(&store, || {
    //     println!("Calling back...");
    //     println!("> Hello World!");
    // });

    // Once we've got that all set up we can then move to the instantiation
    // phase, pairing together a compiled module as well as a set of imports.
    // Note that this is where the wasm `start` function, if any, would run.
    println!("Instantiating module...");
    // let imports = [hello_func.into()];
    // let instance = Instance::new(&store, &module, &imports)?;

    // Next we poke around a bit to extract the `run` function from the module.
    println!("Extracting export...");
    // let run = instance.get_typed_func::<i32, i32>("double")?;
    // let result = run.call(12)?;
    // println!("{}", result);

    let run = instance.get_typed_func::<(), ()>("run")?;
    run.call(())?;

    println!("Done.");
    Ok(())
}

fn main() -> Result<()> {
    let conf =
        thread::Builder::new().name("cool thread name".into());

    let child: thread::JoinHandle<Result<()>> = conf.spawn(move || {
        runtime()?;
        Ok(())
    })?;

    let result = child.join();
    println!("result: {:?}", result);

    Ok(())
}
