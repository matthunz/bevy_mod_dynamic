use bevy_mod_dynamic_client::{App, Runtime};

#[no_mangle]
pub fn main(){
    App::new()
        .add_system(|| {
            todo!()
        })
        .spawn();
}

#[no_mangle]
pub fn run() {
    Runtime::current().tick();
}
