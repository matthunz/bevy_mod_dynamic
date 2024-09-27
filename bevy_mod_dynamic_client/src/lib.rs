use slotmap::{DefaultKey, SlotMap};
use std::{cell::RefCell, rc::Rc, sync::Mutex};

thread_local! {
    static RUNTIME: RefCell<Option<Runtime>> = RefCell::new(None);
}

#[derive(Default)]
struct Inner {
    systems: SlotMap<DefaultKey, Box<dyn FnMut()>>,
}

#[derive(Clone, Default)]
pub struct Runtime {
    inner: Rc<RefCell<Inner>>,
}

impl Runtime {
    pub fn current() -> Self {
        RUNTIME.with(|rt| rt.borrow().as_ref().unwrap().clone())
    }

    pub fn tick(&self) {
        self.inner
            .borrow_mut()
            .systems
            .values_mut()
            .for_each(|f| f());
    }
}

pub struct App {
    rt: Option<Runtime>,
}

impl App {
    pub fn new() -> Self {
        Self {
            rt: Some(Runtime::default()),
        }
    }

    pub fn add_system(&mut self, f: impl FnMut() + 'static) -> &mut Self {
        self.rt
            .as_mut()
            .unwrap()
            .inner
            .borrow_mut()
            .systems
            .insert(Box::new(f));
        self
    }

    pub fn spawn(&mut self) {
        RUNTIME.with(|rt| {
            *rt.borrow_mut() = Some(self.rt.take().unwrap());
        });
    }
}
