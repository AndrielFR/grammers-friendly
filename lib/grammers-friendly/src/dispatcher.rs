// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::pin::pin;

use futures_util::future::{select, Either};
use grammers_client::Client;

use crate::{Handler, Middleware};

/// Dispatcher used to register handlers and middlewares
pub struct Dispatcher {
    handlers: Vec<Handler>,
    middlewares: Vec<Middleware>,
}

impl Dispatcher {
    /// Create a new dispatcher
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            middlewares: Vec::new(),
        }
    }

    /// Add a new handler to the dispatcher
    pub fn add_handler(mut self, handler: Handler) -> Self {
        self.handlers.push(handler);
        self
    }

    /// Add a new middleware to the dispatcher
    pub fn add_middleware(mut self, middleware: Middleware) -> Self {
        self.middlewares.push(middleware);
        self
    }

    /// Run the dispatcher && the bot
    pub async fn run(self, client: Client) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let exit = pin!(async { tokio::signal::ctrl_c().await });
            let update = pin!(async { client.next_update().await });

            let update = match select(exit, update).await {
                Either::Left(_) => break,
                Either::Right((u, _)) => u?,
            };

            let cl = client.clone();
            let upd = update.clone();
            let handlers = self.handlers.clone();
            let middlewares = self.middlewares.clone();
            tokio::task::spawn(async move {
                for handler in handlers.iter() {
                    handler.handle(&cl, &upd.clone().unwrap()).await;
                }

                for middleware in middlewares.iter() {
                    middleware.handle(&cl, &upd.clone().unwrap()).await;
                }
            });
        }

        Ok(())
    }
}
