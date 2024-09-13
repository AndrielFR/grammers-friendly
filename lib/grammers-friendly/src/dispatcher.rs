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

use crate::{traits::Module, Data, Middleware, MiddlewareType, Router};

/// The main dispatcher.
///
/// Receives `modules`, `middlewares` and `routers`.
#[derive(Default)]
pub struct Dispatcher {
    data: Data,
    middlewares: Vec<Middleware>,
    routers: Vec<Router>,
}

impl Dispatcher {
    /// Attach a new middleware to the dispatcher.
    ///
    /// Which will be runned before or after each `handler`.
    pub fn add_middleware(mut self, middleware: Middleware) -> Self {
        self.middlewares.push(middleware);
        self
    }

    /// Attach a new module to the dispatcher.
    ///
    /// Which will be sent a mutable reference for each `middleware` and `handler`.
    ///
    /// Has no effect if added after sub-routers.
    pub fn add_module<M: Module>(mut self, module: M) -> Self {
        self.data.add_module(module);
        self
    }

    /// Attach a new router to the dispatcher.
    ///
    /// Which will be runned after the before `middleware`.
    pub fn add_router(mut self, mut router: Router) -> Self {
        self.data.modules().into_iter().for_each(|module| {
            router.push_module(module);
        });

        self.routers.push(router);
        self
    }

    /// Run the dispatcher.
    ///
    /// Listen to the updates sent by Telegram and distribute them whitin the `routers`.
    pub async fn run(mut self, client: Client) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let exit = pin!(async { tokio::signal::ctrl_c().await });
            let update = pin!(async { client.next_update().await });

            let mut update = match select(exit, update).await {
                Either::Left(_) => break,
                Either::Right((u, _)) => u.unwrap(),
            };

            moro::async_scope!(|scope| {
                let mut client = client.clone();
                let update = &mut update;

                let data = &mut self.data;
                let routers = &mut self.routers;
                let middlewares = &mut self.middlewares;

                scope.spawn(async move {
                    for middleware in middlewares
                        .iter_mut()
                        .filter(|m| m.mtype() == MiddlewareType::Before)
                    {
                        middleware.call(&mut client, update, data).await;
                    }

                    for router in routers.iter_mut() {
                        if router.handle_update(&mut client, update).await {
                            break;
                        }
                    }

                    for middleware in middlewares
                        .iter_mut()
                        .filter(|m| m.mtype() == MiddlewareType::After)
                    {
                        middleware.call(&mut client, update, data).await;
                    }
                });
            })
            .await;
        }

        Ok(())
    }
}
