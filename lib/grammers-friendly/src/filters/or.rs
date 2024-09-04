// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::Arc;

use async_trait::async_trait;
use grammers_client::{Client, Update};

use crate::traits::Filter;

pub struct OrFilter {
    first: Arc<dyn Filter + Send + Sync>,
    other: Arc<dyn Filter + Send + Sync>,
}

impl OrFilter {
    pub fn new(
        first: impl Filter + Send + Sync + 'static,
        other: impl Filter + Send + Sync + 'static,
    ) -> Self {
        Self {
            first: Arc::new(first),
            other: Arc::new(other),
        }
    }
}

#[async_trait]
impl Filter for OrFilter {
    async fn is_ok(&self, client: &Client, update: &Update) -> bool {
        self.first.is_ok(client, update).await || self.other.is_ok(client, update).await
    }
}

pub fn or(
    first: impl Filter + Send + Sync + 'static,
    other: impl Filter + Send + Sync + 'static,
) -> OrFilter {
    OrFilter::new(first, other)
}
