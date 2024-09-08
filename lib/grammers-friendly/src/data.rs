// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::traits::Module;

#[derive(Clone, Default)]
pub struct Data {
    pub(crate) modules: Vec<Box<dyn Module>>,
}

impl Data {
    /// Get a copy of the modules
    pub fn modules(&self) -> Vec<Box<dyn Module>> {
        self.modules.clone()
    }

    /// Attach a new module
    pub(crate) fn add_module(&mut self, module: impl Module + Send + Sync + 'static) {
        self.modules.push(Box::new(module));
    }

    /// Push a new module
    pub(crate) fn push_module(&mut self, module: Box<dyn Module>) {
        self.modules.push(module);
    }

    /// Get a module and downcast it
    pub fn get_module<T: Module>(&self) -> Option<Box<T>> {
        self.modules
            .iter()
            .find_map(|module| module.clone().downcast::<T>().ok())
    }

    /* /// Get a module and downcast it
    /// Unfortunately, it is necessary to use unsafe code
    pub fn get_module<T: Module>(&self) -> Option<Arc<Mutex<T>>> {
        for module in self.modules() {
            if let Ok(m) = module.try_lock() {
                if (*m).is::<T>() {
                    let raw: *const Mutex<dyn Module> = Arc::into_raw(module.clone());
                    let raw: *const Mutex<T> = raw.cast();

                    return Some(unsafe { Arc::from_raw(raw) });
                }
            }
        }

        None
    } */
}
