// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::Arc;

use async_recursion::async_recursion;
use grammers_client::{Client, Update};
use tokio::sync::Mutex;

use crate::{traits::Module, Data, Handler, Middleware};

/// A Router, like a sub-disptacher.
///
/// Receives `modules`, `handlers`, `middlewares` and `sub-routers`.
#[derive(Clone, Default)]
pub struct Router {
    data: Data,
    handlers: Vec<Handler>,
    middlewares: Vec<Arc<Mutex<Middleware>>>,
    sub_routers: Vec<Box<Router>>,
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
        self.middlewares.push(Arc::new(Mutex::new(middleware)));
        self
    }

    /// Attach a new module to the router.
    ///
    /// Which will be sent a mutable reference for each `middleware` and `handler`.
    pub fn add_module<M: Module>(mut self, module: M) -> Self {
        self.data.push_module(Box::new(module));
        self
    }

    /// Attach a new sub-router to the router.
    ///
    /// Which will be runned if the current router don't handle the update.
    pub fn add_sub_router(mut self, sub_router: Router) -> Self {
        self.sub_routers.push(Box::new(sub_router));
        self
    }

    /// Attach a new boxed module to the router.
    ///
    /// Which will be sent a mutable reference for each `middleware` and `handler`.
    pub(crate) fn push_module(&mut self, module: Box<dyn Module>) {
        self.data.push_module(module);
    }

    /// Attach a new boxed middleware to the router.
    ///
    /// Which will be runned before or after each `handler`.
    pub(crate) fn push_middleware(&mut self, middleware: Arc<Mutex<Middleware>>) {
        self.middlewares.push(middleware);
    }

    /// Update sub-routers' data and middlewares.
    pub(crate) fn update_sub_routers(&mut self) {
        self.sub_routers.iter_mut().for_each(|sub_router| {
            self.data.modules.iter().for_each(|module| {
                sub_router.push_module(Box::clone(module));
            });

            self.middlewares.iter().for_each(|middleware| {
                sub_router.push_middleware(Arc::clone(middleware));
            });
        });
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
            let middlewares = &mut self.middlewares;
            let sub_routers = &mut self.sub_routers;

            scope.spawn(async {
                for handler in handlers.iter_mut() {
                    if handler.handle(client, update, data, middlewares).await {
                        update_handled = true;
                        return;
                    }
                }

                for sub_router in sub_routers.iter_mut() {
                    if sub_router.handle_update(client, update).await {
                        update_handled = true;
                        return;
                    }
                }
            });
        })
        .await;

        update_handled
    }
}
