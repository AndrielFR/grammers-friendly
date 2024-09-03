// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::Arc;

use grammers_client::{Client, Update};

use crate::traits::Filter;

pub struct AndFilter {
    first: Arc<dyn Filter + Send + Sync>,
    second: Arc<dyn Filter + Send + Sync>,
}

impl AndFilter {
    pub fn new(
        first: impl Filter + Send + Sync + 'static,
        second: impl Filter + Send + Sync + 'static,
    ) -> Self {
        Self {
            first: Arc::new(first),
            second: Arc::new(second),
        }
    }
}

impl Filter for AndFilter {
    fn is_ok(&self, client: &Client, update: &Update) -> bool {
        self.first.is_ok(client, update) && self.second.is_ok(client, update)
    }
}

pub fn and(
    first: impl Filter + Send + Sync + 'static,
    second: impl Filter + Send + Sync + 'static,
) -> AndFilter {
    AndFilter::new(first, second)
}
