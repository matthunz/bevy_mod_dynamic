use bevy::{prelude::*, utils::HashMap};
use serde::Serialize;
use serde_json::Value;
use slotmap::{DefaultKey, SlotMap};
use std::{any::TypeId, borrow::Cow, path::Path};
use wasmer::{imports, Function, Instance, Module, Store};

struct DynamicPlugin {
    module: Module,
    update: Function,
}

#[derive(Default, Resource)]
pub struct Runtime {
    store: Store,
    modules: SlotMap<DefaultKey, DynamicPlugin>,
    json: HashMap<TypeId, Value>,
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
    dbg!(&rt.json);
    rt.tick();
}

fn update_json<T: Component + Serialize>(component_query: Query<Ref<T>>, mut rt: ResMut<Runtime>) {
    for component in &component_query {
        if component.is_changed() {
            rt.json.insert(
                TypeId::of::<T>(),
                serde_json::to_value(&*component).unwrap(),
            );
        }
    }
}

#[derive(Component, Serialize)]
struct Health(i32);

fn main() {
    App::new()
        .add_plugins(RuntimePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update_json::<Health>)
        .run();
}

fn setup(mut commands: Commands, mut rt: ResMut<Runtime>) {
    rt.load("target/wasm32-unknown-unknown/debug/example.wasm");

    commands.spawn(Health(100));
}
