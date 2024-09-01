// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{pin::Pin, sync::Arc};

use dyn_clone::DynClone;
use futures_util::Future;
use grammers_client::{Client, Update};

use crate::Filter;

/// Handler
#[derive(Clone)]
pub struct Handler {
    name: String,
    func: Box<dyn AsyncFn + Send + Sync>,
    filters: Vec<Arc<dyn Filter + Send + Sync>>,
}

impl Handler {
    pub fn new(name: &str, func: impl AsyncFn + Send + Sync + 'static) -> Self {
        Self {
            name: name.to_string(),
            func: Box::new(func),
            filters: Vec::new(),
        }
    }

    pub fn filter(mut self, filter: impl Filter + 'static + Send + Sync) -> Self {
        self.filters.push(Arc::new(filter));
        self
    }

    pub fn filters(mut self, filters: Vec<impl Filter + 'static + Send + Sync>) -> Self {
        let _ = filters
            .into_iter()
            .map(|f| self.filters.push(Arc::new(f)))
            .collect::<Vec<_>>();
        self
    }

    pub async fn handle(&self, client: &Client, update: &Update) {
        for filter in self.filters.iter() {
            if !filter.is_ok(client, update) {
                return;
            }

            self.func
                .call(client.clone(), update.clone())
                .await
                .unwrap();
        }
    }
}

type PinBox =
    Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + Sync + 'static>>;

pub trait AsyncFn: DynClone {
    fn call(&self, client: Client, update: Update) -> PinBox;
}

impl<T, F> AsyncFn for T
where
    T: Fn(Client, Update) -> F,
    T: DynClone,
    F: Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + Sync + 'static,
    // T: Send + Sync + 'static,
{
    fn call(&self, client: Client, update: Update) -> PinBox {
        Box::pin(self(client, update))
    }
}

dyn_clone::clone_trait_object!(AsyncFn);
