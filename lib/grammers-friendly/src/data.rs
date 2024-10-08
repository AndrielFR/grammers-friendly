// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::traits::Module;

/// Data.
///
/// Actually, it just stores the modules.
#[derive(Clone, Default)]
pub struct Data {
    pub(crate) modules: Vec<Box<dyn Module>>,
}

impl Data {
    /// Get a copy of the modules.
    pub fn modules(&self) -> Vec<Box<dyn Module>> {
        self.modules.clone()
    }

    /// Attach a new boxed module.
    pub(crate) fn push_module(&mut self, module: Box<dyn Module>) {
        self.modules.push(module);
    }

    /// Get a module and downcast it.
    ///
    /// # Example
    ///
    /// ```
    /// # async fn f(client: &mut grammers_client::Client, update: &mut grammers_client::Update, data: &mut crate:: data) -> Result<(), Box<dyn std::error::Error>> {
    /// let t = data.get_modules::<T>().unwrap();
    ///
    /// # Ok()
    /// # }
    /// ```
    pub fn get_module<M: Module>(&self) -> Option<Box<M>> {
        self.modules
            .iter()
            .find_map(|module| Box::clone(module).downcast::<M>().ok())
    }
}
