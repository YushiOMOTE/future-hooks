use futures;
use futures::prelude::*;
use futures::task_local;

use crate::hooks::{FutureHookExt, Hook, Hooked};
use std::cell::RefCell;

task_local! {
    static LOG_PREFICES: RefCell<Vec<String>> = RefCell::new(Vec::new())
}

pub fn prefix<F>(f: F)
where
    F: Fn(&str),
{
    LOG_PREFICES.with(|ps| f(ps.borrow().last().unwrap()))
}

pub struct Logger {
    prefix: String,
}

impl Hook for Logger {
    fn on_prepoll(&mut self) {
        LOG_PREFICES.with(|ps| {
            ps.borrow_mut().push(self.prefix.clone());
        });
    }

    fn on_postpoll(&mut self) {
        LOG_PREFICES.with(|ps| {
            ps.borrow_mut().pop();
        });
    }
}

#[macro_export]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => {
        future_hooks::prefix(|pfx| {
            let args = format_args!($($arg)+);
            log::info!(target: $target, "{}: {}", pfx, args);
        })
    };
    ($($arg:tt)+) => {
        future_hooks::prefix(|pfx| {
            let args = format_args!($($arg)+);
            log::info!("{}: {}", pfx, args);
        })
    };
}

#[macro_export]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)+) => {
        future_hooks::prefix(|pfx| {
            let args = format_args!($($arg)+);
            log::warn!(target: $target, "{}: {}", pfx, args);
        })
    };
    ($($arg:tt)+) => {
        future_hooks::prefix(|pfx| {
            let args = format_args!($($arg)+);
            log::warn!("{}: {}", pfx, args);
        })
    };
}

#[macro_export]
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => {
        future_hooks::prefix(|pfx| {
            let args = format_args!($($arg)+);
            log::error!(target: $target, "{}: {}", pfx, args);
        })
    };
    ($($arg:tt)+) => {
        future_hooks::prefix(|pfx| {
            let args = format_args!($($arg)+);
            log::error!("{}: {}", pfx, args);
        })
    };
}

#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => {
        future_hooks::prefix(|pfx| {
            let args = format_args!($($arg)+);
            log::debug!(target: $target, "{}: {}", pfx, args);
        })
    };
    ($($arg:tt)+) => {
        future_hooks::prefix(|pfx| {
            let args = format_args!($($arg)+);
            log::debug!("{}: {}", pfx, args);
        })
    };
}

#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => {
        future_hooks::prefix(|pfx| {
            let args = format_args!($($arg)+);
            log::trace!(target: $target, "{}: {}", pfx, args);
        })
    };
    ($($arg:tt)+) => {
        future_hooks::prefix(|pfx| {
            let args = format_args!($($arg)+);
            log::trace!("{}: {}", pfx, args);
        })
    };
}

pub trait FutureLogExt: Future {
    fn log(self, pfx: &str) -> Hooked<Self, Logger>
    where
        Self: Sized,
    {
        self.hook(Logger { prefix: pfx.into() })
    }
}

impl<T> FutureLogExt for T where T: Future {}
