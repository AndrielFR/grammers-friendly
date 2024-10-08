// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{pin::pin, sync::Arc};

use futures_util::future::{select, Either};
use grammers_client::{types::Chat, Client};
use tokio::sync::Mutex;

use crate::{
    traits::{GetSender, Module},
    Data, Middleware, Router,
};

/// The main dispatcher.
///
/// Receives `modules`, `middlewares` and `routers`.
#[derive(Default)]
pub struct Dispatcher {
    data: Data,
    middlewares: Vec<Arc<Mutex<Middleware>>>,
    routers: Vec<Router>,

    ignore_updates_from_self: bool,
}

impl Dispatcher {
    /// Attach a new middleware to the dispatcher.
    ///
    /// Which will be runned before or after each `handler`.
    ///
    /// Has no effect if added after sub-routers.
    pub fn add_middleware(mut self, middleware: Middleware) -> Self {
        self.middlewares.push(Arc::new(Mutex::new(middleware)));
        self
    }

    /// Attach a new module to the dispatcher.
    ///
    /// Which will be sent a mutable reference for each `middleware` and `handler`.
    ///
    /// Has no effect if added after sub-routers.
    pub fn add_module<M: Module>(mut self, module: M) -> Self {
        self.data.push_module(Box::new(module));
        self
    }

    /// Attach a new router to the dispatcher.
    ///
    /// Which will be runned after the before `middleware`.
    pub fn add_router(mut self, mut router: Router) -> Self {
        self.data.modules.iter().for_each(|module| {
            router.push_module(Box::clone(module));
        });

        self.middlewares.iter().for_each(|middleware| {
            router.push_middleware(Arc::clone(middleware));
        });

        router.update_sub_routers();

        self.routers.push(router);
        self
    }

    /// Ignore updates from self.
    ///
    /// Telegram sends the update of the invocation made by the user.
    /// If disabled the updates will be handled.
    ///
    /// `true` -> ignore.
    /// `false` -> handle it (default).
    pub fn ignore_updates_from_self(mut self, value: bool) -> Self {
        self.ignore_updates_from_self = value;
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
                Either::Right((u, _)) => u?,
            };

            moro::async_scope!(|scope| {
                let mut client = client.clone();
                let update = &mut update;

                let routers = &mut self.routers;

                scope.spawn(async move {
                    if self.ignore_updates_from_self {
                        if let Some(sender) = update.get_sender() {
                            if let Chat::User(user) = sender {
                                if user.is_self() {
                                    return;
                                }
                            }
                        }
                    }

                    for router in routers.iter_mut() {
                        if router.handle_update(&mut client, update).await {
                            break;
                        }
                    }
                });
            })
            .await;
        }

        Ok(())
    }
}
