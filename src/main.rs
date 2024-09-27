use bevy::prelude::*;
use slotmap::{DefaultKey, SlotMap};
use std::path::Path;
use wasmer::{imports, Function, Instance, Module, Store};

struct DynamicPlugin {
    module: Module,
    update: Function,
}

#[derive(Default, Resource)]
pub struct Runtime {
    store: Store,
    modules: SlotMap<DefaultKey, DynamicPlugin>,
}

impl Runtime {
    pub fn load(&mut self, path: impl AsRef<Path>) {
        let wasm_bytes = std::fs::read(path).unwrap();

        let module = Module::new(&self.store, wasm_bytes).unwrap();
        let import_object = imports! {};

        let instance = Instance::new(&mut self.store, &module, &import_object).unwrap();

        instance
            .exports
            .get_function("main")
            .unwrap()
            .call(&mut self.store, &[])
            .unwrap();

        let update = instance.exports.get_function("run").unwrap().clone();

        self.modules.insert(DynamicPlugin { module, update });
    }

    pub fn tick(&mut self) {
        for (_, plugin) in self.modules.iter_mut() {
            dbg!(plugin.update.call(&mut self.store, &[]).unwrap());
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
    rt.load("target/wasm32-unknown-unknown/debug/example.wasm");
}
