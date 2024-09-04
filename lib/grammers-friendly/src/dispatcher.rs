// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{pin::pin, sync::Arc};

use async_recursion::async_recursion;
use futures_util::future::{select, Either};
use grammers_client::{Client, Update};

use crate::{traits::Module, Handler, Middleware};

/// Dispatcher used to register handlers and middlewares
pub struct Dispatcher {
    handlers: Vec<Handler>,
    middlewares: Vec<Middleware>,
    modules: Vec<Arc<dyn Module>>,
    routers: Vec<Arc<Dispatcher>>,
}

impl Dispatcher {
    /// Create a new dispatcher
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            middlewares: Vec::new(),
            modules: Vec::new(),
            routers: Vec::new(),
        }
    }

    /// Attach a new handler to the dispatcher
    pub fn add_handler(mut self, handler: Handler) -> Self {
        self.handlers.push(handler);
        self
    }

    /// Attach a new middleware to the dispatcher
    pub fn add_middleware(mut self, middleware: Middleware) -> Self {
        self.middlewares.push(middleware);
        self
    }

    /// Attach a new module to the dispatcher
    pub fn add_module(mut self, module: impl Module + Send + Sync + 'static) -> Self {
        self.modules.push(Arc::new(module));
        self
    }

    /// Attach a new router (sub-disptacher) to the dispatcher
    pub fn add_router(mut self, router: Dispatcher) -> Self {
        self.routers.push(Arc::new(router));
        self
    }

    /// Run the main dispatcher
    pub async fn run(self, client: Client) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let exit = pin!(async { tokio::signal::ctrl_c().await });
            let update = pin!(async { client.next_update().await });

            let update = match select(exit, update).await {
                Either::Left(_) => break,
                Either::Right((u, _)) => u?,
            };

            self.handle_update(client.clone(), update.unwrap()).await?;
        }

        Ok(())
    }

    #[async_recursion]
    pub(crate) async fn handle_update(
        &self,
        client: Client,
        update: Update,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.handlers.is_empty() || !self.middlewares.is_empty() || !self.modules.is_empty() {
            let mut client = client.clone();
            let mut update = update.clone();
            let handlers = self.handlers.clone();
            let middlewares = self.middlewares.clone();
            let modules = self.modules.clone();
            tokio::task::spawn(async move {
                for module in modules.iter() {
                    module.ante_call(&mut client, &mut update).await.unwrap();
                }

                for handler in handlers.iter() {
                    if handler.handle(&client, &update, &modules).await {
                        return;
                    }
                }

                for middleware in middlewares.iter() {
                    if middleware.handle(&client, &update, &modules).await {
                        return;
                    }
                }

                for module in modules.iter() {
                    module.post_call(&mut client, &mut update).await.unwrap();
                }
            });
        }

        if !self.routers.is_empty() {
            for router in self.routers.iter() {
                router
                    .handle_update(client.clone(), update.clone())
                    .await
                    .unwrap();
            }
        }

        Ok(())
    }
}
