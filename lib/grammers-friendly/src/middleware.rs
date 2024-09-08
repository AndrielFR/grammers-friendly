// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use grammers_client::{Client, Update};

use crate::{traits::MiddlewareImpl, Data, Handler};

/// Middleware
#[derive(Clone)]
pub struct Middleware {
    mid: Box<dyn MiddlewareImpl + Send + Sync + 'static>,
    handlers: Vec<Handler>,
}

impl Middleware {
    /// Create a new middleware
    pub fn new(mid: impl MiddlewareImpl + Send + Sync + 'static) -> Self {
        Self {
            mid: Box::new(mid),
            handlers: Vec::new(),
        }
    }

    /// Add a handler to the middleware
    pub fn add_handler(mut self, handler: Handler) -> Self {
        self.handlers.push(handler);
        self
    }

    /// Before each handler, run the middleware first
    pub async fn handle(&self, client: &Client, update: &Update, data: &Data) -> bool {
        if !self.handlers.is_empty() {
            let r = self.mid.call(client.clone(), update.clone()).await;
            if let Err(e) = r {
                log::error!("Error running middleware: {:?}", e);
                return false;
            }
        }

        for handler in self.handlers.iter() {
            if handler.handle(client, update, data).await {
                return true;
            }
        }

        false
    }
}
