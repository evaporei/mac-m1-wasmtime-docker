use anyhow::Result;
use std::thread;
use wasmtime::*;

fn runtime() -> Result<()> {
    println!("Initializing...");

    let mut config = Config::new();
    config.strategy(Strategy::Cranelift).unwrap(); // probably what causes the issue
    config.interruptable(true);
    config.cranelift_nan_canonicalization(true);
    config.cranelift_opt_level(OptLevel::None);

    let engine = Engine::new(&config)?;
    let store = Store::new(&engine);

    let mut linker = Linker::new(&store);
    linker.func("host", "hello", || println!("hello"))?;

    let wat = r#"
      (module
        (func $hello (import "host" "hello"))
        (func (export "run") (call $hello))
      )
    "#;
    let module = Module::new(store.engine(), wat)?;
    let instance = linker.instantiate(&module)?;

    // Useless logs just left because they were present
    // in the failing example in README.md, but they
    // have no relation to the issue.
    println!("Compiling module...");
    println!("Creating callback...");
    println!("Instantiating module...");
    println!("Extracting export...");

    let run = instance.get_typed_func::<(), ()>("run")?;
    run.call(())?;

    println!("Done."); // does not happen
    Ok(())
}

fn main() -> Result<()> {
    let conf = thread::Builder::new().name("cool thread name".into());

    let child: thread::JoinHandle<Result<()>> = conf.spawn(move || {
        runtime()?;
        Ok(())
    })?;

    let result = child.join();
    println!("result: {:?}", result); // does not happen

    Ok(())
}
