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

pub struct NotFilter {
    filter: Arc<dyn Filter + Send + Sync>,
}

impl NotFilter {
    pub fn new(filter: impl Filter + Send + Sync + 'static) -> Self {
        Self {
            filter: Arc::new(filter),
        }
    }
}

#[async_trait]
impl Filter for NotFilter {
    async fn is_ok(&self, client: &Client, update: &Update) -> bool {
        !self.filter.is_ok(client, update).await
    }
}

pub fn not(filter: impl Filter + Send + Sync + 'static) -> NotFilter {
    NotFilter::new(filter)
}
