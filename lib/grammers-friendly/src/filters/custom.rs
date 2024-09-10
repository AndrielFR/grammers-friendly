// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{pin::Pin, sync::Arc};

use async_trait::async_trait;
use futures_util::Future;
use grammers_client::{Client, Update};

use crate::traits::Filter;

type PinBox = Pin<Box<dyn Future<Output = bool> + Send>>;

pub trait CustomFilterFn: Send + Sync + 'static {
    fn call(&self, client: Client, update: Update) -> PinBox;
}

impl<T, F> CustomFilterFn for T
where
    T: Fn(Client, Update) -> F + Send + Sync + 'static,
    F: Future<Output = bool> + Send + 'static,
{
    fn call(&self, client: Client, update: Update) -> PinBox {
        Box::pin((self)(client, update))
    }
}

/// A custom filter, accepts a async closure
#[derive(Clone)]
pub struct CustomFilter {
    func: Arc<dyn CustomFilterFn>,
}

impl CustomFilter {
    pub fn new<F>(func: F) -> Self
    where
        F: CustomFilterFn,
    {
        Self {
            func: Arc::new(func),
        }
    }
}

#[async_trait]
impl Filter for CustomFilter {
    async fn is_ok(&mut self, client: &Client, update: &Update) -> bool {
        self.func.call(client.clone(), update.clone()).await
    }
}

pub fn custom<F>(func: F) -> CustomFilter
where
    F: CustomFilterFn,
{
    CustomFilter::new(func)
}
