use anyhow::Result;
#[cfg(target_os = "hermit")]
use hermit as _;
use log::debug;
use simple_logger::SimpleLogger;
use wasmi::*;

fn main() -> Result<()> {
	SimpleLogger::new().init().unwrap();

	println!("Start WASI demo!");

	// First step is to create the Wasm execution engine with some config.
	// In this example we are using the default configuration.
	let engine = Engine::default();

	// TODO: dirty workaround to get the WebAssembly module into
	// the VM. Find a way to inject the `.wasm` file into the VM
	// using another way
	debug!("Create Module");
	let module_bytes = include_bytes!("../wasm/fib.wasm");
	let module = Module::new(&engine, &mut &module_bytes[..])?;

	// All Wasm objects operate within the context of a `Store`.
	// Each `Store` has a type parameter to store host-specific data,
	// which in this case we are using `42` for.
	type HostState = u32;
	let mut store = Store::new(&engine, 42);

	// In order to create Wasm module instances and link their imports
	// and exports we require a `Linker`.
	let linker = <Linker<HostState>>::new(&engine);
	let instance = linker.instantiate(&mut store, &module)?.start(&mut store)?;

	debug!("Try to find function fibonacci");
	let fibonacci = instance.get_typed_func::<u64, u64>(&store, "fibonacci")?;

	// And finally we can call the wasm!
	debug!("Call function fibonacci");
	let result = fibonacci.call(&mut store, 30)?;
	println!("fibonacci(30) = {}", result);
	assert!(
		result == 832040,
		"Error in the calculation of fibonacci(30) "
	);

	Ok(())
}
