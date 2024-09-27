use wasmer::{imports, Instance, Module, Store, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_bytes = std::fs::read("target/wasm32-unknown-unknown/debug/example.wasm")?;
    let mut store = Store::default();

    let module = Module::new(&store, wasm_bytes)?;
    let import_object = imports! {};

    let instance = Instance::new(&mut store, &module, &import_object)?;
    let add_function = instance.exports.get_function("add")?;

    let result = add_function.call(&mut store, &[Value::I32(5), Value::I32(7)])?;
    if let Value::I32(sum) = result[0] {
        println!("Result of add: {}", sum);
    }

    Ok(())
}
