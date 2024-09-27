use bevy::prelude::*;
use slotmap::{DefaultKey, SlotMap};
use std::{
    path::Path,
    sync::{mpsc, Mutex},
    thread,
};
use wasmer::{imports, Function, Instance, Module, Store, Value};

struct DynamicPlugin {
    module: Module,
    systems: Vec<Function>,
}

#[derive(Default, Resource)]
pub struct Runtime {
    store: Store,
    modules: SlotMap<DefaultKey, DynamicPlugin>,
}

impl Runtime {
    pub fn load(&mut self, path: impl AsRef<Path>, systems: &[&str]) {
        let wasm_bytes = std::fs::read(path).unwrap();

        let module = Module::new(&self.store, wasm_bytes).unwrap();
        let import_object = imports! {};

        let instance = Instance::new(&mut self.store, &module, &import_object).unwrap();

        let fns = systems
            .iter()
            .map(|name| instance.exports.get_function(name).unwrap().clone())
            .collect();

        self.modules.insert(DynamicPlugin {
            module,
            systems: fns,
        });
    }

    pub fn tick(&mut self) {
        for (_, plugin) in self.modules.iter_mut() {
            for system in &plugin.systems {
                dbg!(system
                    .call(&mut self.store, &[Value::I32(5), Value::I32(7)])
                    .unwrap());
            }
        }
    }
}

pub struct RuntimePlugin;

impl Plugin for RuntimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Runtime>()
            .add_systems(Update, tick_runtime);
    }
}

fn tick_runtime(mut rt: ResMut<Runtime>) {
    rt.tick();
}

fn main() {
    App::new()
        .add_plugins(RuntimePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut rt: ResMut<Runtime>) {
    rt.load("target/wasm32-unknown-unknown/debug/example.wasm", &["add"]);
}
