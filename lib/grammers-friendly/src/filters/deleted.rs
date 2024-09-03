// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use grammers_client::{Client, Update};

use crate::traits::Filter;

pub struct DeletedFilter;

impl DeletedFilter {
    pub fn new() -> Self {
        Self
    }
}

impl Filter for DeletedFilter {
    fn is_ok(&self, _client: &Client, update: &Update) -> bool {
        matches!(update, Update::MessageDeleted(_))
    }
}

pub fn deleted() -> DeletedFilter {
    DeletedFilter::new()
}
