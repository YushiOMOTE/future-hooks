use futures;
use futures::prelude::*;
use log::info;
use std::time::Instant;

use crate::hooks::{FutureHookExt, Hook, Hooked};

pub struct Latency {
    name: String,
    start: Option<Instant>,
}

impl Hook for Latency {
    fn on_prepoll(&mut self) {
        self.start.get_or_insert_with(|| Instant::now());
    }

    fn on_ready(&mut self) {
        let latency = Instant::now() - self.start.unwrap();
        info!("{}: latency {:?}", self.name, latency);
    }
}

pub trait FutureLatencyExt: Future {
    fn latency(self, name: &str) -> Hooked<Self, Latency>
    where
        Self: Sized,
    {
        self.hook(Latency {
            name: name.into(),
            start: None,
        })
    }
}

impl<T> FutureLatencyExt for T where T: Future {}
