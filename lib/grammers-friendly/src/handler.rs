// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::Arc;

use grammers_client::{Client, Update};

use crate::traits::{AsyncFn, Filter, Module};

/// Handler
#[derive(Clone)]
pub struct Handler {
    name: String,
    func: Box<dyn AsyncFn + Send + Sync>,
    filter: Arc<dyn Filter + Send + Sync>,
    // TODO: update_type: UpdateType,
}

impl Handler {
    /// Create a new handler
    pub fn new(
        name: &str,
        func: impl AsyncFn + Send + Sync + 'static,
        filter: impl Filter + Send + Sync + 'static,
    ) -> Self {
        Self {
            name: name.to_string(),
            func: Box::new(func),
            filter: Arc::new(filter),
        }
    }

    /// If filters pass, run the func
    pub async fn handle(&self, client: &Client, update: &Update, modules: &Vec<Arc<dyn Module>>) {
        if !self.filter.is_ok(client, update).await {
            return;
        }

        self.func
            .call(client.clone(), update.clone(), modules.clone())
            .await
            .unwrap();
    }
}
