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

pub struct Dispatcher {
    client: Client,
    handlers: Vec<Handler>,
    middlewares: Vec<Middleware>,
}

impl Dispatcher {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            handlers: Vec::new(),
            middlewares: Vec::new(),
        }
    }

    pub fn handler(mut self, handler: Handler) -> Self {
        self.handlers.push(handler);
        self
    }

    pub fn middleware(mut self, middleware: Middleware) -> Self {
        self.middlewares.push(middleware);
        self
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let exit = pin!(async { tokio::signal::ctrl_c().await });
            let upd = pin!(async { self.client.next_update().await });

            let update = match select(exit, upd).await {
                Either::Left(_) => break,
                Either::Right((u, _)) => u?,
            };

            let client = self.client.clone();
            let handlers = self.handlers.clone();
            tokio::task::spawn(async move {
                for handler in handlers.iter() {
                    handler.handle(&client, &update.clone().unwrap()).await;
                }
            });
        }

        Ok(())
    }
}
