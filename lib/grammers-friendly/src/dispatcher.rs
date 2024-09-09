// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::pin::pin;

use async_recursion::async_recursion;
use futures_util::future::{select, Either};
use grammers_client::{Client, Update};

use crate::{traits::Module, Data, Handler, Middleware, MiddlewareType};

/// Dispatcher used to register handlers, middlewares and routers
#[derive(Default)]
pub struct Dispatcher {
    data: Data,
    handlers: Vec<Handler>,
    middlewares: Vec<Middleware>,
    routers: Vec<Dispatcher>,
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
    pub fn add_module<M: Module>(mut self, module: M) -> Self {
        self.data.add_module(module);
        self
    }

    /// Attach a new router (sub-disptacher) to the dispatcher
    pub fn add_router(mut self, mut router: Dispatcher) -> Self {
        self.data.modules().into_iter().for_each(|module| {
            router.data.push_module(module);
        });

        self.routers.push(router);
        self
    }

    /// Run the main dispatcher
    pub async fn run(mut self, client: Client) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let exit = pin!(async { tokio::signal::ctrl_c().await });
            let update = pin!(async { client.next_update().await });

            let update = match select(exit, update).await {
                Either::Left(_) => break,
                Either::Right((u, _)) => u.unwrap(),
            };

            let r = handle_update(
                client.clone(),
                update,
                &mut self.data,
                &mut self.routers,
                &mut self.handlers,
                &mut self.middlewares,
            )
            .await;
            if let Err(e) = r {
                log::error!("Error dispatching: {}", e);
            }
        }

        Ok(())
    }
}

/// Handle the updates sent by Telegram
#[async_recursion]
pub(crate) async fn handle_update(
    client: Client,
    update: Update,
    data: &mut Data,
    routers: &mut [Dispatcher],
    handlers: &mut [Handler],
    middlewares: &mut [Middleware],
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut update_handled = false;

    moro::async_scope!(|scope| {
        let mut client = client.clone();
        let mut update = update.clone();

        scope.spawn(async move {
            for middleware in middlewares
                .iter_mut()
                .filter(|m| m.mtype() == MiddlewareType::Before)
            {
                let r = middleware.call(&mut client, &mut update, data).await;
                if let Err(e) = r {
                    log::error!("Error running middleware: {}", e);
                }
            }

            for handler in handlers.iter_mut() {
                if handler.handle(&client, &update, data).await {
                    update_handled = true;
                    break;
                }
            }

            for middleware in middlewares
                .iter_mut()
                .filter(|m| m.mtype() == MiddlewareType::After)
            {
                let r = middleware.call(&mut client, &mut update, data).await;
                if let Err(e) = r {
                    log::error!("Error running middleware: {}", e);
                }
            }

            if !update_handled {
                for router in routers.iter_mut() {
                    let r = handle_update(
                        client.clone(),
                        update.clone(),
                        &mut router.data,
                        &mut router.routers,
                        &mut router.handlers,
                        &mut router.middlewares,
                    )
                    .await;
                    if let Ok(true) = r {
                        break;
                    } else if let Err(e) = r {
                        log::error!("Error running router: {}", e);
                    }
                }
            }
        });
    })
    .await;

    Ok(update_handled)
}
