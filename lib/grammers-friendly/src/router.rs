// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use async_recursion::async_recursion;
use grammers_client::{Client, Update};

use crate::{traits::Module, Data, Handler, Middleware};

/// A Router, like a sub-disptacher.
///
/// Receives `modules`, `handlers`, `middlewares` and `sub-routers`.
#[derive(Clone, Default)]
pub struct Router {
    data: Data,
    handlers: Vec<Handler>,
    middlewares: Vec<Middleware>,
    sub_routers: Vec<Router>,
}

impl Router {
    /// Attach a new handler to the router.
    ///
    /// Which will be runned in sequence in which they were added.
    ///
    /// Will stop running when a handler's filters match.
    pub fn add_handler(mut self, handler: Handler) -> Self {
        self.handlers.push(handler);
        self
    }

    /// Attach a new middleware to the router.
    ///
    /// Which will be runned before or after each `handler`.
    pub fn add_middleware(mut self, middleware: Middleware) -> Self {
        self.middlewares.push(middleware);
        self
    }

    /// Attach a new unboxed module to the router.
    ///
    /// Which will be sent a mutable reference for each `middleware` and `handler`.
    pub fn add_module<M: Module>(mut self, module: M) -> Self {
        self.data.add_module(module);
        self
    }

    /// Attach a new middleware to the router.
    ///
    /// Which will be runned before or after each `handler`.
    pub(crate) fn push_middleware(&mut self, middleware: Middleware) {
        self.middlewares.push(middleware);
    }

    /// Attach a new boxed module to the router.
    ///
    /// Which will be sent a mutable reference for each `middleware` and `handler`.
    pub(crate) fn push_module(&mut self, module: Box<dyn Module>) {
        self.data.push_module(module);
    }

    /// Attach a new sub-router to the router.
    ///
    /// Which will be runned if the current router don't handle the update.
    pub fn add_sub_router(mut self, mut sub_router: Router) -> Self {
        // Send a clone of each module to the sub-router
        self.data.modules.clone().into_iter().for_each(|module| {
            sub_router.push_module(module);
        });

        // Send a clone of each middleware to the sub-router
        self.middlewares.clone().into_iter().for_each(|middleware| {
            sub_router.push_middleware(middleware);
        });

        self.sub_routers.push(sub_router);
        self
    }

    /// Handle the update sent by Telegram.
    ///
    /// Starts by the before-type middlewares, handlers, after-types middlewares and if not handled
    /// Send to the sub-routers.
    #[async_recursion]
    pub(crate) async fn handle_update(&mut self, client: &mut Client, update: &mut Update) -> bool {
        let mut update_handled = false;

        moro::async_scope!(|scope| {
            let data = &mut self.data;
            let handlers = &mut self.handlers;
            let sub_routers = &mut self.sub_routers;
            let middlewares = &mut self.middlewares;

            scope.spawn(async move {
                for handler in handlers.iter_mut() {
                    if handler.handle(client, update, data, middlewares).await {
                        update_handled = true;
                        break;
                    }
                }

                if !update_handled {
                    for sub_router in sub_routers.iter_mut() {
                        if sub_router.handle_update(client, update).await {
                            break;
                        }
                    }
                }
            });
        })
        .await;

        update_handled
    }
}
