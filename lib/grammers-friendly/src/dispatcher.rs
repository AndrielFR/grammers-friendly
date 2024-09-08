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

use crate::{traits::Module, Data, Handler, Middleware};

/// Dispatcher used to register handlers and middlewares
#[derive(Default)]
pub struct Dispatcher {
    data: Data,
    handlers: Vec<Handler>,
    middlewares: Vec<Middleware>,
    routers: Vec<Arc<Dispatcher>>,
}

impl Dispatcher {
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
        self.data.add_module(module);
        self
    }

    /// Attach a new router (sub-disptacher) to the dispatcher
    pub fn add_router(mut self, mut router: Dispatcher) -> Self {
        self.data.modules().iter().for_each(|module| {
            router.data.push_module(module.clone());
        });

        self.routers.push(Arc::new(router));
        self
    }

    /// Run the main dispatcher
    pub async fn run(self, client: Client) -> Result<(), Box<dyn std::error::Error>> {
        moro::async_scope!(|scope| {
            loop {
                let exit = pin!(async { tokio::signal::ctrl_c().await });
                let update = pin!(async { client.next_update().await });

                let update = match select(exit, update).await {
                    Either::Left(_) => break,
                    Either::Right((u, _)) => u.unwrap(),
                };

                scope.spawn(async {
                    self.handle_update(client.clone(), update).await.unwrap();
                });
            }
        })
        .await;

        Ok(())
    }

    #[async_recursion]
    pub(crate) async fn handle_update(
        &self,
        client: Client,
        update: Update,
    ) -> Result<(), Box<dyn std::error::Error>> {
        moro::async_scope!(|scope| {
            if !self.handlers.is_empty()
                || !self.middlewares.is_empty()
                || !self.data.modules.is_empty()
            {
                let mut client = client.clone();
                let mut update = update.clone();
                let mut data = self.data.clone();
                let handlers = &self.handlers;
                let middlewares = &self.middlewares;
                scope.spawn(async move {
                    for module in (data.modules).iter_mut() {
                        let _ = module.ante_call(&mut client, &mut update).await;
                    }

                    for handler in handlers.iter() {
                        if handler.handle(&client, &update, &data).await {
                            return;
                        }
                    }

                    for middleware in middlewares.iter() {
                        if middleware.handle(&client, &update, &data).await {
                            return;
                        }
                    }

                    for module in (data.modules).iter_mut() {
                        let _ = module.post_call(&mut client, &mut update).await;
                    }
                });
            }

            if !self.routers.is_empty() {
                for router in self.routers.iter() {
                    scope
                        .spawn(async {
                            router
                                .handle_update(client.clone(), update.clone())
                                .await
                                .unwrap();
                        })
                        .await;
                }
            }
        })
        .await;

        Ok(())
    }
}
