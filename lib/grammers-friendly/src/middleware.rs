// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::Arc;

use grammers_client::{Client, Update};

use crate::{traits::MiddlewareImpl, Data, Handler};

/// Middleware
#[derive(Clone)]
pub struct Middleware {
    mid: Arc<dyn MiddlewareImpl>,
    handlers: Vec<Handler>,
    type_: MiddleWareType,
}

impl Middleware {
    /// Create a new middleware
    pub fn new(mid: impl MiddlewareImpl, type_: MiddleWareType) -> Self {
        Self {
            mid: Arc::new(mid),
            handlers: Vec::new(),
            type_,
        }
    }

    /// Add a handler to the middleware
    pub fn add_handler(mut self, handler: Handler) -> Self {
        self.handlers.push(handler);
        self
    }

    /// Before each handler, run the middleware first
    pub async fn handle(
        &mut self,
        client: &mut Client,
        update: &mut Update,
        data: &mut Data,
    ) -> bool {
        let mut result = false;

        for handler in self.handlers.iter_mut() {
            if self.type_ == MiddleWareType::Before {
                let r = self.mid.call(client, update, data).await;
                if let Err(e) = r {
                    log::error!("Error running middleware: {:?}", e);
                }
            }

            if handler.handle(client, update, data).await {
                result = true;
            }

            if self.type_ == MiddleWareType::After {
                let r = self.mid.call(client, update, data).await;
                if let Err(e) = r {
                    log::error!("Error running middleware: {:?}", e);
                }
            }
        }

        result
    }
}

#[derive(Clone, PartialEq)]
pub enum MiddleWareType {
    Before,
    After,
}
